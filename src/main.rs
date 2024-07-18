use base64::prelude::*;
use serde_json::Value;
use std::{
    collections::HashMap,
    env,
    io::{BufReader, Write},
    rc::Rc,
    time::Instant,
};
use thiserror::Error;
use token::Token;

mod api;
mod token;

#[derive(Debug, Error)]
enum RedditAuthError {
    #[error("environment variables not set")]
    NoEnvVariables,

    #[error("cannot reach api")]
    RequestError(#[from] Box<ureq::Error>),

    #[error("unexpected response from api")]
    DeserializeError(#[from] serde_json::Error),
}

struct Reddit {
    headers: HashMap<String, String>,
    token: Token,

    limit: u32,
    base_url: String,
}

impl Reddit {
    fn new() -> Self {
        let headers = HashMap::from_iter([
            (
                "User-Agent".to_owned(),
                "Rust: Trends v0.1.0 by u/molivo10".to_owned(),
            ),
            ("Accept-Encoding".to_owned(), "gzip, deflate, br".to_owned()),
            ("Accept".to_owned(), "*/*".to_owned()),
        ]);

        Reddit {
            headers,
            token: Token::new(),

            limit: 100,
            base_url: "https://oauth.reddit.com".to_owned(),
        }
    }

    fn update_headers(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    fn set_headers(&self, mut req: ureq::Request) -> ureq::Request {
        for (key, value) in self.headers.iter() {
            req = req.set(key, value);
        }

        req
    }

    fn authorize(&mut self) -> Result<(), RedditAuthError> {
        let (Ok(client_id), Ok(client_secret), Ok(username), Ok(password)) = (
            env::var("REDDIT_CLIENT_ID"),
            env::var("REDDIT_CLIENT_SECRET"),
            env::var("REDDIT_USERNAME"),
            env::var("REDDIT_PASSWORD"),
        ) else {
            return Err(RedditAuthError::NoEnvVariables);
        };

        let mut req = ureq::post("https://www.reddit.com/api/v1/access_token");

        req = self.set_headers(req);

        let encoded = BASE64_STANDARD_NO_PAD.encode(format!("{}:{}", client_id, client_secret));
        let authorization = format!("Basic {}", encoded);

        req = req.set("Authorization", &authorization);

        let res = req
            .send_form(&[
                ("grant_type", "password"),
                ("username", &username),
                ("password", &password),
            ])
            .map_err(Box::new)?;

        let token: token::Token = serde_json::from_reader(res.into_reader())?;

        self.update_headers(
            "Authorization".to_owned(),
            format!("Bearer {}", token.access_token),
        );

        self.token = token;

        Ok(())
    }

    fn user_profile(&self, username: &'static str) {
        let url = format!("/user/{username}/submitted");

        let full_url = self.base_url.clone() + &url;

        let mut query: HashMap<String, String> = HashMap::from([
            ("limit".to_owned(), "100".to_owned()),
            ("context".to_owned(), "2".to_owned()),
            ("show".to_owned(), "given".to_owned()),
            ("sort".to_owned(), "new".to_owned()),
            ("t".to_owned(), "all".to_owned()),
            ("type".to_owned(), "all".to_owned()),
            ("raw_json".to_owned(), "1".to_owned()),
        ]);

        loop {
            let mut req = ureq::get(&full_url);

            req = self.set_headers(req);

            let res = req
                .query_pairs(query.iter().map(|(k, v)| (k.as_str(), v.as_str())))
                .call()
                .unwrap();

            dbg!(&res);

            let data: api::ProfileResponse = serde_json::from_reader(res.into_reader()).unwrap();

            if data.data.after.is_null() {
                break;
            } else {
                dbg!(&data.data.after);
                let after = data.data.after.as_str().unwrap().to_owned();
                query.insert("after".to_owned(), after.clone());
            }
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    let now = Instant::now();
    let mut r = Reddit::new();
    r.authorize().unwrap();
    r.user_profile("Avereniect");

    let done = now.elapsed().as_millis();
    println!("done in {done}ms");
}
