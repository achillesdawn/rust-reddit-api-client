use base64::prelude::*;
use reddit::RedditError;
use reqwest::{Client, StatusCode, header};
use std::{collections::HashMap, env};
use tracing::debug;

use crate::{
    api::{Post, Profile, RedditApiResonse},
    token::{self, Token},
};

mod download;
mod gif_client;
mod image_client;

pub use download::get_post_images;

pub struct Reddit {
    token: Token,
    base_url: String,
    client: Client,

    timer: tokio::time::Interval,
}

impl Reddit {
    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            // .user_agent("Rust: Followers v0.1.0 by u/ghostofmikael")
            .user_agent("Rust: trends v0.1.0 by u/molivo10")
            .build()
            .unwrap();

        let period = std::time::Duration::from_secs_f32(1. / 100.);

        let timer = tokio::time::interval(period);

        Reddit {
            token: Token::new(),
            client,
            base_url: "https://oauth.reddit.com".to_owned(),
            timer,
        }
    }

    fn encode_authorization(client_id: String, client_secret: String) -> String {
        let encoded = BASE64_STANDARD_NO_PAD.encode(format!("{}:{}", client_id, client_secret));
        let encoded_auth = format!("Basic {}", encoded);
        encoded_auth
    }

    pub async fn authorize(&mut self) -> Result<(), RedditError> {
        let (Ok(client_id), Ok(client_secret), Ok(username), Ok(password)) = (
            env::var("REDDIT_CLIENT_ID"),
            env::var("REDDIT_CLIENT_SECRET"),
            env::var("REDDIT_USERNAME"),
            env::var("REDDIT_PASSWORD"),
        ) else {
            return Err(RedditError::NoEnvVariables);
        };

        let encoded_auth = Reddit::encode_authorization(client_id, client_secret);

        let res = self
            .client
            .post("https://www.reddit.com/api/v1/access_token")
            .header("Authorization", encoded_auth)
            .form(&[
                ("grant_type", "password"),
                ("username", &username),
                ("password", &password),
            ])
            .send()
            .await
            .unwrap();

        let res = match res.error_for_status() {
            Ok(res) => res,
            Err(err) => {
                if let Some(StatusCode::UNAUTHORIZED) = err.status() {
                    println!("Unauthorized");
                    return Err(RedditError::Unauthorized);
                } else {
                    return Err(RedditError::RequestError(Box::new(err)));
                }
            }
        };

        let bytes = res.bytes().await.unwrap();

        let token: token::Token = match serde_json::from_reader(bytes.as_ref()) {
            Ok(token) => token,
            Err(err) => return Err(RedditError::DeserializeError(err)),
        };

        debug!(?token);

        let mut default_headers = header::HeaderMap::new();
        default_headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", token.access_token)).unwrap(),
        );

        let client = reqwest::ClientBuilder::new()
            .user_agent("Rust: trends v0.1.0 by u/molivo10")
            .default_headers(default_headers)
            .build()
            .unwrap();

        self.client = client;
        self.token = token;

        Ok(())
    }

    pub async fn subreddit(
        &self,
        subreddit_name: &str,
    ) -> Result<RedditApiResonse<Post>, RedditError> {
        let url = format!("/r/{subreddit_name}/new");
        let full_url = self.base_url.clone() + &url;

        let query: HashMap<String, String> = HashMap::from([
            ("limit".to_owned(), "100".to_owned()),
            ("show".to_owned(), "all".to_owned()),
        ]);

        let res = self
            .client
            .get(full_url)
            .query(&query)
            .send()
            .await
            .unwrap();

        let bytes = res.bytes().await.unwrap();

        let data: RedditApiResonse<Post> = serde_json::from_reader(bytes.as_ref())?;

        Ok(data)
    }

    pub async fn user_profile_latest(
        &mut self,
        username: String,
    ) -> Result<Vec<Post>, RedditError> {
        let url = format!("/user/{username}/submitted");

        let full_url = self.base_url.clone() + &url;

        let mut posts = Vec::new();

        let query: HashMap<String, String> = HashMap::from([
            ("limit".to_owned(), "100".to_owned()),
            ("context".to_owned(), "2".to_owned()),
            ("show".to_owned(), "given".to_owned()),
            ("sort".to_owned(), "new".to_owned()),
            ("t".to_owned(), "links".to_owned()),
            ("type".to_owned(), "all".to_owned()),
            ("raw_json".to_owned(), "1".to_owned()),
        ]);

        self.timer.tick().await;

        let res = self
            .client
            .get(full_url.clone())
            .query(&query)
            .send()
            .await
            .map_err(Box::new)?;

        if res.status() != StatusCode::OK {
            let bytes = res.bytes().await.unwrap();
            let raw = std::str::from_utf8(&bytes).unwrap();
            dbg!(raw);
            return Err(RedditError::Unauthorized);
        }

        let bytes = res.bytes().await.unwrap();

        // let mut file = std::fs::File::create("test.json").unwrap();
        // file.write_all(bytes.as_ref()).unwrap();

        let deserializer = &mut serde_json::Deserializer::from_reader(bytes.as_ref());

        let data: RedditApiResonse<Post> = match serde_path_to_error::deserialize(deserializer) {
            Ok(data) => data,
            Err(err) => {
                dbg!(&err);
                let raw = std::str::from_utf8(&bytes).unwrap();
                dbg!(raw);
                let path = err.path().to_string();
                dbg!(path);
                return Err(RedditError::NoEnvVariables);
            }
        };

        posts.extend(data.data.children.into_iter().map(|child| child.data));

        Ok(posts)
    }

    pub async fn user_profile(&mut self, username: String) -> Result<Vec<Post>, RedditError> {
        let url = format!("/user/{username}/submitted");

        let full_url = self.base_url.clone() + &url;

        let mut posts = Vec::new();

        let mut query: HashMap<String, String> = HashMap::from([
            ("limit".to_owned(), "100".to_owned()),
            ("context".to_owned(), "2".to_owned()),
            ("show".to_owned(), "given".to_owned()),
            ("sort".to_owned(), "new".to_owned()),
            ("t".to_owned(), "links".to_owned()),
            ("type".to_owned(), "all".to_owned()),
            ("raw_json".to_owned(), "1".to_owned()),
        ]);

        loop {
            self.timer.tick().await;

            let res = self
                .client
                .get(full_url.clone())
                .query(&query)
                .send()
                .await
                .map_err(Box::new)?;

            if res.status() != StatusCode::OK {
                let bytes = res.bytes().await.unwrap();
                let raw = std::str::from_utf8(&bytes).unwrap();
                dbg!(raw);
                return Err(RedditError::Unauthorized);
            }

            let bytes = res.bytes().await.unwrap();

            let deserializer = &mut serde_json::Deserializer::from_reader(bytes.as_ref());

            let data: RedditApiResonse<Post> = match serde_path_to_error::deserialize(deserializer)
            {
                Ok(data) => data,
                Err(err) => {
                    dbg!(&err);
                    let raw = std::str::from_utf8(&bytes).unwrap();
                    dbg!(raw);
                    let path = err.path().to_string();
                    dbg!(path);
                    return Err(RedditError::NoEnvVariables);
                }
            };

            posts.extend(data.data.children.into_iter().map(|child| child.data));

            if data.data.after.is_null() {
                break;
            } else {
                dbg!(&data.data.after);
                let after = data.data.after.as_str().unwrap().to_owned();
                query.insert("after".to_owned(), after.clone());
            }
        }

        Ok(posts)
    }

    pub async fn following(&self) -> Result<Vec<Profile>, RedditError> {
        let url = "/subreddits/mine/subscriber".to_string();
        let full_url = self.base_url.clone() + &url;

        let mut results = Vec::new();

        let mut query: HashMap<String, String> = HashMap::from([
            ("limit".to_owned(), "100".to_owned()),
            ("show".to_owned(), "all".to_owned()),
        ]);

        loop {
            let res = self
                .client
                .get(full_url.clone())
                .query(&query)
                .send()
                .await
                .map_err(Box::new)?;

            let bytes = res.bytes().await.unwrap();

            let deserializer = &mut serde_json::Deserializer::from_reader(bytes.as_ref());

            let data: RedditApiResonse<Profile> =
                match serde_path_to_error::deserialize(deserializer) {
                    Ok(data) => data,
                    Err(err) => {
                        dbg!(&err);
                        let path = err.path().to_string();
                        dbg!(path);
                        return Err(RedditError::NoEnvVariables);
                    }
                };

            results.extend(data.data.children.into_iter().map(|child| child.data));

            if data.data.after.is_null() {
                break;
            } else {
                let after = data.data.after.as_str().unwrap().to_owned();
                query.insert("after".to_owned(), after.clone());
            }
        }

        Ok(results)
    }
}
