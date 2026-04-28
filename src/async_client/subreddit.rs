use crate::api::Post;
use crate::error::RedditError;
use crate::{
    api::{RedditApiResonse, Subreddit},
    async_client::Reddit,
};
use eyre::Context;
use serde_json::Value;

impl Reddit {
    pub async fn subreddit(
        &mut self,
        subreddit_name: &str,
    ) -> eyre::Result<RedditApiResonse<Post>> {
        let mut url = self
            .base_url
            .join(&format!("/r/{subreddit_name}/new"))
            .wrap_err("could not create url")?;

        url.query_pairs_mut()
            .append_pair("limit", "100")
            .append_pair("show", "all")
            .finish();

        let req = reqwest::Request::new(reqwest::Method::GET, url);

        let bytes = self.handle_request(req).await?;

        let data: RedditApiResonse<Post> = serde_json::from_reader(bytes.as_ref())?;

        Ok(data)
    }

    /// Searches for subreddits matching the given query.
    /// Uses GET /subreddits/search
    pub async fn search_subreddits(
        &self,
        query: &str,
        limit: Option<u32>,
    ) -> eyre::Result<RedditApiResonse<Subreddit>> {
        let url = format!("{}/subreddits/search", self.base_url);

        let mut query_params = vec![("q", query.to_string())];

        if let Some(l) = limit {
            query_params.push(("limit", l.to_string()));
        }

        let res = self
            .client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .map_err(|e| RedditError::RequestError(Box::new(e)))?;

        let bytes = res
            .bytes()
            .await
            .map_err(|e| RedditError::RequestError(Box::new(e)))?;

        let s = std::str::from_utf8(&bytes).wrap_err("could not deserialzie response")?;

        dbg!(s);

        serde_json::from_slice(&bytes).wrap_err("could not deserialize reddit response")
    }

    /// Autocomplete style search for subreddits.
    /// Uses GET /api/search_subreddits
    pub async fn autocomplete_subreddits(
        &self,
        query: &str,
    ) -> eyre::Result<RedditApiResonse<Subreddit>> {
        let url = format!("{}/api/search_subreddits", self.base_url);

        let query_params = [("query", query)];

        let res = self
            .client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .map_err(|e| RedditError::RequestError(Box::new(e)))?;

        let bytes = res
            .bytes()
            .await
            .map_err(|e| RedditError::RequestError(Box::new(e)))?;

        serde_json::from_slice(&bytes).wrap_err("could not deserialize autocomplete bytes response")
    }

    /// Lightweight search for subreddit names.
    /// Uses GET /api/search_reddit_names
    pub async fn search_reddit_names(&self, query: &str) -> Result<Vec<String>, RedditError> {
        let url = format!("{}/api/search_reddit_names", self.base_url);
        let query_params = [("query", query)];

        let res = self
            .client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .map_err(|e| RedditError::RequestError(Box::new(e)))?;

        let bytes = res
            .bytes()
            .await
            .map_err(|e| RedditError::RequestError(Box::new(e)))?;

        let json: Value =
            serde_json::from_reader(bytes.as_ref()).map_err(RedditError::DeserializeError)?;

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
    async fn test_search_subreddit_names() -> Result<()> {
        let client = Reddit::new().await?;

        let result = client.search_subreddits("human", Some(100)).await?;

        dbg!(result);

        Ok(())
    }
}
