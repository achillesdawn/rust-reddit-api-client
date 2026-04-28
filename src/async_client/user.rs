use eyre::Context;
use reqwest::Method;

use crate::api::{Post, RedditApiResonse};

impl super::Reddit {
    pub async fn user_posts_latest(&mut self, username: String) -> eyre::Result<Vec<Post>> {
        let mut url = self
            .base_url
            .join(&format!("/user/{username}/submitted"))
            .wrap_err("could not create url")?;

        url.query_pairs_mut()
            .extend_pairs([
                ("limit", "100"),
                ("context", "2"),
                ("show", "given"),
                ("sort", "new"),
                ("t", "links"),
                ("type", "all"),
                ("raw_json", "1"),
            ])
            .finish();

        let req = reqwest::Request::new(Method::GET, url);

        let mut posts = Vec::new();

        let b = self.handle_request(req).await?;

        let deserializer = &mut serde_json::Deserializer::from_slice(&b);

        let data: RedditApiResonse<Post> = match serde_path_to_error::deserialize(deserializer) {
            Ok(data) => data,
            Err(err) => {
                dbg!(&err);
                let raw = std::str::from_utf8(&b).unwrap();
                dbg!(raw);
                let path = err.path().to_string();
                dbg!(path);
                return Err(eyre::eyre!("could not deserialize bytes response"));
            }
        };

        posts.extend(data.data.children.into_iter().map(|child| child.data));

        Ok(posts)
    }

    pub async fn user_posts(&mut self, username: String) -> eyre::Result<Vec<Post>> {
        let mut url = self
            .base_url
            .join(&format!("/user/{username}/submitted"))
            .wrap_err("could not create url")?;

        let query = [
            ("limit", "100"),
            ("context", "2"),
            ("show", "given"),
            ("sort", "new"),
            ("t", "links"),
            ("type", "all"),
            ("raw_json", "1"),
        ];

        url.query_pairs_mut().extend_pairs(query).finish();

        let mut posts = Vec::new();

        loop {
            let req = reqwest::Request::new(Method::GET, url.clone());

            let b = self.handle_request(req).await?;

            let deserializer = &mut serde_json::Deserializer::from_slice(&b);

            let data: RedditApiResonse<Post> = match serde_path_to_error::deserialize(deserializer)
            {
                Ok(data) => data,
                Err(err) => {
                    dbg!(&err);
                    let raw = std::str::from_utf8(&b).unwrap();
                    dbg!(raw);
                    let path = err.path().to_string();
                    dbg!(path);
                    return Err(eyre::eyre!("could not deserialize bytes response"));
                }
            };

            posts.extend(data.data.children.into_iter().map(|child| child.data));

            if data.data.after.is_null() {
                break;
            } else {
                let after = data.data.after.as_str().unwrap().to_owned();
                url.query_pairs_mut()
                    .clear()
                    .extend_pairs(query)
                    .append_pair("after", &after)
                    .finish();
            }
        }

        Ok(posts)
    }
}
