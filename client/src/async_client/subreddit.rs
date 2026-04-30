use crate::api::Post;
use crate::{
    api::{RedditApiResponse, Subreddit},
    async_client::Reddit,
};
use eyre::Context;
use reqwest::Method;
use serde_json::Value;

impl Reddit {
    pub async fn subreddit_about(&mut self, subreddit: &str) -> eyre::Result<Subreddit> {
        let url = self
            .base_url
            .join(&format!("/r/{}/about", subreddit))
            .wrap_err("could not create url")?;

        let req = reqwest::Request::new(Method::GET, url);

        let data: crate::api::Child<Subreddit> = self.request_and_deserialize(req).await?;

        Ok(data.data)
    }

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

        let data: RedditApiResponse<Post> = self.request_and_deserialize(req).await?;

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

            let data: RedditApiResponse<Post> = self.request_and_deserialize(req).await?;

            posts.extend(data.data.children.into_iter().map(|child| child.data));

            if let Some(after) = data.data.after {
                url.query_pairs_mut()
                    .clear()
                    .extend_pairs(query)
                    .append_pair("after", &after)
                    .finish();
            } else {
                break;
            }
        }

        Ok(posts)
    }

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

        let data: RedditApiResponse<Subreddit> = self.request_and_deserialize(req).await?;

        Ok(data
            .data
            .children
            .into_iter()
            .map(|child| child.data)
            .collect())
    }

    // return a list of 10 autocomplete subreddit suggestions
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

        let data: RedditApiResponse<Subreddit> = self.request_and_deserialize(req).await?;

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

        let json: Value = self.request_and_deserialize(req).await?;

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

    pub async fn subreddits_popular(&mut self, limit: Option<u32>) -> eyre::Result<Vec<Subreddit>> {
        let mut url = self
            .base_url
            .join("/subreddits/popular")
            .wrap_err("could not create url")?;

        if let Some(n) = limit {
            url.query_pairs_mut().append_pair("limit", &n.to_string());
        }

        let req = reqwest::Request::new(Method::GET, url);

        let data: RedditApiResponse<Subreddit> = self.request_and_deserialize(req).await?;

        Ok(data
            .data
            .children
            .into_iter()
            .map(|child| child.data)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use super::*;

    #[tokio::test]
    async fn test_subreddits_popular() -> Result<()> {
        let mut client = Reddit::new().await?;

        let result = client.subreddits_popular(Some(100)).await?;

        result.iter().for_each(|i| {
            println!("{}", i.title);
        });

        assert!(!result.is_empty());
        dbg!(result.len());

        Ok(())
    }

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

        let result = client.subreddit_posts_latest("selfhosted").await?;

        result.iter().for_each(|i| {
            println!("{} - {}", i.title, i.author);
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_subreddit_about() -> Result<()> {
        let mut client = Reddit::new().await?;

        let sub = client.subreddit_about("rust").await?;

        dbg!(sub);

        Ok(())
    }
}
