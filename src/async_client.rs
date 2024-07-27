use base64::prelude::*;
use reddit::RedditError;
use reqwest::{header, Client, IntoUrl, StatusCode};
use std::{collections::HashMap, env, io::Write, path::PathBuf, str::FromStr};

use crate::{
    api::{Post, RedditApiResonse},
    token::{self, Token},
};

pub struct Reddit {
    token: Token,
    base_url: String,
    client: Client,
}

impl Reddit {
    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            // .user_agent("Rust: Followers v0.1.0 by u/ghostofmikael")
            .user_agent("Rust: trends v0.1.0 by u/molivo10")
            .build()
            .unwrap();

        Reddit {
            token: Token::new(),
            client,
            base_url: "https://oauth.reddit.com".to_owned(),
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

        dbg!(&token);

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

    pub async fn subreddit(&self, subreddit_name: &str) -> Result<RedditApiResonse, RedditError> {
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

        let data: RedditApiResonse = serde_json::from_reader(bytes.as_ref())?;

        Ok(data)
    }

    pub async fn user_profile(&self, username: String) -> Result<Vec<Post>, RedditError> {
        let url = format!("/user/{username}/submitted");

        let full_url = self.base_url.clone() + &url;

        let mut result = Vec::new();

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
            let res = self
                .client
                .get(full_url.clone())
                .query(&query)
                .send()
                .await
                .map_err(Box::new)?;

            let bytes = res.bytes().await.unwrap();

            let deserializer = &mut serde_json::Deserializer::from_reader(bytes.as_ref());

            let data: RedditApiResonse = match serde_path_to_error::deserialize(deserializer) {
                Ok(data) => data,
                Err(err) => {
                    dbg!(&err);
                    let path = err.path().to_string();
                    dbg!(path);
                    return Err(RedditError::NoEnvVariables);
                }
            };

            result.extend(data.data.children.into_iter().map(|child| child.data));

            if data.data.after.is_null() {
                break;
            } else {
                dbg!(&data.data.after);
                let after = data.data.after.as_str().unwrap().to_owned();
                query.insert("after".to_owned(), after.clone());
            }
        }

        Ok(result)
    }

    pub async fn following(&self) -> Result<(), RedditError> {
        let url = format!("/subreddits/mine/subscriber");
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
            .map_err(Box::new)?;

        dbg!(&res);

        let bytes = res.bytes().await.unwrap();

        let data: serde_json::Value = serde_json::from_reader(bytes.as_ref()).unwrap();
        let file = std::fs::File::create("result.json").unwrap();

        serde_json::to_writer(file, &data).unwrap();

        Ok(())
    }
}

async fn download(url: impl IntoUrl, path: PathBuf) {
    if path.exists() {
        return
    }

    let res = reqwest::get(url).await.unwrap();
    let bytes = res.bytes().await.unwrap();

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}

pub async fn get_post_images(post: Post) {
    let output_dir = PathBuf::from_str("assets").unwrap();
    let mut output_dir = std::path::absolute(output_dir).unwrap();

    output_dir.push(&post.author);
    if post.title.len() > 255 {
        let name: String = post.title.chars().take(200).collect();
        output_dir.push(name);
    } else {
        output_dir.push(&post.title);
    }

    if !output_dir.exists() {
        match std::fs::create_dir_all(&output_dir) {
            Ok(_) => {}
            Err(err) => {
                dbg!(&output_dir);
                dbg!(&err);
            }
        };
    }

    let mut join_set = tokio::task::JoinSet::new();

    if let Some(metadata) = post.media_metadata {
        for (key, value) in metadata.into_iter() {
            let mut image_path = output_dir.clone();
            image_path.push(key);

            if value.o.is_none() {
                continue;
            }

            let image = value.s.as_ref().unwrap();

            if image.u.is_none() {
                continue;
            }
            let url = image.u.as_ref().unwrap().clone();
            join_set.spawn(download(url, image_path));
        }
    }

    if let Some(preview) = post.preview {
        for image in preview.images {
            let image_name = image.source.url.rsplit_once("/").unwrap().1;
            let image_name = image_name.split_once("?").unwrap().0;

            let mut image_path = output_dir.clone();
            image_path.push(image_name);

            join_set.spawn(download(image.source.url, image_path));
        }
    }

    while let Some(res) = join_set.join_next().await {
        if res.is_err() {
            dbg!(res.err());
        }
    }
}
