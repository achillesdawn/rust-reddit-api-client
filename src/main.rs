use base64::prelude::*;
use std::{collections::HashMap, env};
use thiserror::Error;
use token::Token;

mod token;
mod api;

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

        let mut req = ureq::get(&full_url);

        req = self.set_headers(req);

        let query: Vec<(&str, &str)> = vec![
            ("limit", "100"),
            ("context", "2"),
            ("show", "given"),
            ("sort", "new"),
            ("t", "all"),
            ("type", "all"),
            ("raw_json", "1"),
        ];

        let res = req.query_pairs(query).call().unwrap();

        dbg!(&res);

        let data: api::Profile = serde_json::from_reader(res.into_reader()).unwrap();

        for child in data.data.children {
            dbg!(child.data);
        }

        // let file = std::fs::File::create("result.json").unwrap();
        
        // serde_json::to_writer(file, &data).unwrap();
    }
}

fn main() {
    dotenv::dotenv().ok();

    let mut r = Reddit::new();
    r.authorize().unwrap();
    r.user_profile("username");
}
