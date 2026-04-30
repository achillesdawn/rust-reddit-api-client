use eyre::Context;
use reqwest::{Client, Method};

use crate::api::{ApiResponse, Kind};

mod auth;
mod download;
mod gif_client;
mod image_client;
mod profile;
mod subreddit;
mod token;
mod user;

pub use download::get_post_images;
use token::Token;

pub struct Reddit {
    token: Token,
    base_url: url::Url,
    client: Client,

    timer: tokio::time::Interval,
}

impl Reddit {
    fn build_client(token: Token) -> reqwest::Client {
        reqwest::ClientBuilder::new()
            // .user_agent("Rust: Followers v0.1.0 by u/ghostofmikael")
            .user_agent("Rust: trends v0.1.0 by u/molivo10")
            .default_headers(token.into())
            .build()
            .unwrap()
    }

    pub async fn new() -> eyre::Result<Self> {
        let period = std::time::Duration::from_secs_f32(1. / 100.);

        let timer = tokio::time::interval(period);

        let token = Reddit::authenticate().await?;

        let client = Reddit::build_client(token.clone());

        Ok(Reddit {
            token,
            client,
            base_url: url::Url::parse("https://oauth.reddit.com").expect("could no parse base url"),
            timer,
        })
    }

    async fn handle_request(&mut self, req: reqwest::Request) -> eyre::Result<bytes::Bytes> {
        self.timer.tick().await;

        if self.auth_token_expired() {
            self.re_authenticate().await?;
        }

        let r = self
            .client
            .execute(req)
            .await
            .wrap_err("could not execute request")?
            .error_for_status()
            .wrap_err("status error")?;

        r.bytes().await.wrap_err("could not read response bytes")
    }

    pub async fn request_and_deserialize<T>(&mut self, req: reqwest::Request) -> eyre::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let b = self.handle_request(req).await?;

        // let s = std::str::from_utf8(&b).unwrap();
        // dbg!(s);

        let deserializer = &mut serde_json::Deserializer::from_slice(&b);

        let data: T = match serde_path_to_error::deserialize(deserializer) {
            Ok(data) => data,
            Err(err) => {
                dbg!(&err);
                let raw = std::str::from_utf8(&b).unwrap_or("invalid utf8");
                dbg!(raw);
                let path = err.path().to_string();
                dbg!(path);
                return Err(eyre::eyre!("could not deserialize bytes response"));
            }
        };

        Ok(data)
    }

    async fn paginated(&mut self, url: url::Url, limit: Option<usize>) -> eyre::Result<Vec<Kind>> {
        let mut results = Vec::new();
        let mut after: Option<String> = None;

        loop {
            let url = {
                let mut url = url.clone();
                if let Some(after) = after {
                    url.query_pairs_mut().append_pair("after", &after);
                }
                url
            };

            let req = reqwest::Request::new(Method::GET, url);

            let data: ApiResponse = self.request_and_deserialize(req).await?;

            results.extend(data.data.children);

            if let Some(next_after) = data.data.after {
                if let Some(limit) = limit
                    && results.len() >= limit
                {
                    break;
                }
                after = Some(next_after);
            } else {
                break;
            }
        }

        Ok(results)
    }
}
