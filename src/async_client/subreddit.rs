use crate::api::Post;
use crate::{
    api::{RedditApiResonse, Subreddit},
    async_client::Reddit,
};
use eyre::Context;
use reqwest::Method;
use serde_json::Value;

impl Reddit {
    pub async fn subreddit_posts_latest(
        &mut self,
        subreddit_name: &str,
    ) -> eyre::Result<Vec<Post>> {
        let mut url = self
            .base_url
            .join(&format!("/r/{subreddit_name}/new"))
            .wrap_err("could not create url")?;

        url.query_pairs_mut()
            .extend_pairs([("limit", "100"), ("show", "all"), ("raw_json", "1")])
            .finish();

        let req = reqwest::Request::new(Method::GET, url);

        let bytes = self.handle_request(req).await?;

        let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);

        let data: RedditApiResonse<Post> = match serde_path_to_error::deserialize(deserializer) {
            Ok(data) => data,
            Err(err) => {
                dbg!(&err);
                let raw = std::str::from_utf8(&bytes).unwrap_or("invalid utf8");
                dbg!(raw);
                let path = err.path().to_string();
                dbg!(path);
                return Err(eyre::eyre!("could not deserialize bytes response"));
            }
        };

        Ok(data
            .data
            .children
            .into_iter()
            .map(|child| child.data)
            .collect())
    }

    pub async fn subreddit_posts(&mut self, subreddit_name: &str) -> eyre::Result<Vec<Post>> {
        let mut url = self
            .base_url
            .join(&format!("/r/{subreddit_name}/new"))
            .wrap_err("could not create url")?;

        let query = [("limit", "100"), ("show", "all"), ("raw_json", "1")];

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
                    let raw = std::str::from_utf8(&b).unwrap_or("invalid utf8");
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

    /// Searches for subreddits matching the given query.
    /// Uses GET /subreddits/search
    pub async fn search_subreddits(
        &mut self,
        query: &str,
        limit: Option<u32>,
    ) -> eyre::Result<Vec<Subreddit>> {
        let mut url = self
            .base_url
            .join("/subreddits/search")
            .wrap_err("could not create url")?;

        url.query_pairs_mut().append_pair("q", query);

        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        }

        let req = reqwest::Request::new(Method::GET, url);

        let bytes = self.handle_request(req).await?;

        let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);

        let data: RedditApiResonse<Subreddit> = match serde_path_to_error::deserialize(deserializer)
        {
            Ok(data) => data,
            Err(err) => {
                dbg!(&err);
                let raw = std::str::from_utf8(&bytes).unwrap_or("invalid utf8");
                dbg!(raw);
                let path = err.path().to_string();
                dbg!(path);
                return Err(eyre::eyre!("could not deserialize bytes response"));
            }
        };

        Ok(data
            .data
            .children
            .into_iter()
            .map(|child| child.data)
            .collect())
    }

    /// Autocomplete style search for subreddits.
    /// Uses GET /api/subreddit_autocomplete_v2
    pub async fn autocomplete_subreddits(&mut self, query: &str) -> eyre::Result<Vec<Subreddit>> {
        let mut url = self
            .base_url
            .join("/api/subreddit_autocomplete_v2")
            .wrap_err("could not create url")?;

        url.query_pairs_mut()
            .extend_pairs([
                ("query", query),
                ("limit", "10"),
                ("include_profiles", "false"),
                ("include_over_18", "false"),
            ])
            .finish();

        let req = reqwest::Request::new(Method::GET, url);

        let bytes = self.handle_request(req).await?;

        let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);

        let data: RedditApiResonse<Subreddit> = match serde_path_to_error::deserialize(deserializer)
        {
            Ok(data) => data,
            Err(err) => {
                dbg!(&err);
                let raw = std::str::from_utf8(&bytes).unwrap_or("invalid utf8");
                dbg!(raw);
                let path = err.path().to_string();
                dbg!(path);
                return Err(eyre::eyre!("could not deserialize bytes response"));
            }
        };

        Ok(data
            .data
            .children
            .into_iter()
            .map(|child| child.data)
            .collect())
    }

    pub async fn search_reddit_names(&mut self, query: &str) -> eyre::Result<Vec<String>> {
        let mut url = self
            .base_url
            .join("/api/search_reddit_names")
            .wrap_err("could not create url")?;

        url.query_pairs_mut()
            .extend_pairs([
                ("query", query),
                ("include_over_18", "false"),
                ("exact", "false"),
            ])
            .finish();

        let req = reqwest::Request::new(Method::GET, url);

        let bytes = self.handle_request(req).await?;

        let json: Value = serde_json::from_slice(&bytes).wrap_err("could not deserialize json")?;

        // This endpoint returns {"names": ["name1", "name2", ...]}
        if let Some(names) = json.get("names").and_then(|v| v.as_array()) {
            let names_vec = names
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            Ok(names_vec)
        } else {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use super::*;

    #[tokio::test]
    async fn test_search_subreddits() -> Result<()> {
        let mut client = Reddit::new().await?;

        let result = client.search_subreddits("human", Some(100)).await?;

        result.iter().for_each(|i| {
            println!("{}", i.title);
        });

        dbg!(result.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_autocomplete_subreddits() -> Result<()> {
        let mut client = Reddit::new().await?;

        let result = client.autocomplete_subreddits("deep").await?;

        result.iter().for_each(|i| {
            println!("{}", i.title);
        });

        dbg!(result.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_search_reddit_names_subreddits() -> Result<()> {
        let mut client = Reddit::new().await?;

        let result = client.search_reddit_names("art").await?;

        dbg!(&result);

        dbg!(result.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_subreddit_posts_latest() -> Result<()> {
        let mut client = Reddit::new().await?;

        let result = client.subreddit_posts_latest("rust").await?;

        result.iter().for_each(|i| {
            println!("{}", i.title);
        });

        Ok(())
    }
}
