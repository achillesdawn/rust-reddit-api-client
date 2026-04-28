use eyre::Context;
use reqwest::{Client, Method};

use crate::{
    api::{Profile, RedditApiResonse},
    token::Token,
};

mod auth;
mod download;
mod gif_client;
mod image_client;
mod subreddit;
mod user;

pub use download::get_post_images;

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

    pub async fn following(&mut self) -> eyre::Result<Vec<Profile>> {
        let mut url = self
            .base_url
            .join("/subreddits/mine/subscriber")
            .wrap_err("could not create url")?;

        url.query_pairs_mut()
            .extend_pairs([("limit", "100"), ("show", "all")])
            .finish();

        let mut results = Vec::new();

        loop {
            let req = reqwest::Request::new(Method::GET, url.clone());

            let b = self.handle_request(req).await?;

            let deserializer = &mut serde_json::Deserializer::from_slice(&b);

            let data: RedditApiResonse<Profile> =
                match serde_path_to_error::deserialize(deserializer) {
                    Ok(data) => data,
                    Err(err) => {
                        dbg!(&err);
                        let path = err.path().to_string();
                        dbg!(path);
                        return Err(eyre::eyre!("could not deserialize bytes response"));
                    }
                };

            results.extend(data.data.children.into_iter().map(|child| child.data));

            if data.data.after.is_null() {
                break;
            } else {
                let after = data.data.after.as_str().unwrap().to_owned();
                url.query_pairs_mut().append_pair("after", &after).finish();
            }
        }

        Ok(results)
    }
}
