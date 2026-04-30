use crate::api::enums::SubredditSortType;
use crate::api::{ApiResponse, BASE_URL, Kind};
use crate::{
    api::{RedditApiResponse, Subreddit},
    async_client::Reddit,
};
use eyre::Context;
use reqwest::Method;
use serde_json::Value;

fn create_subreddit_url(subreddit: &str, sort_type: SubredditSortType) -> url::Url {
    let mut url = BASE_URL
        .join(&format!("/r/{subreddit}/{sort_type}"))
        .expect("could not create url");

    todo!()
}

impl Reddit {
    pub async fn subreddit_about(&mut self, subreddit: &str) -> eyre::Result<Subreddit> {
        let url = BASE_URL
            .join(&format!("/r/{}/about", subreddit))
            .wrap_err("could not create url")?;

        let req = reqwest::Request::new(Method::GET, url);

        let data: crate::api::Child<Subreddit> = self.request_and_deserialize(req).await?;

        Ok(data.data)
    }

    // fn create_subreddit_url() -> url::Url {}

    // pub async fn subreddit_posts_latest(
    //     &mut self,
    //     subreddit_name: &str,
    //     endpoint: Endpoint,
    //     sort: SortType,
    //     sort_time: Option<SortTime>,
    //     limit: Option<usize>,
    // ) -> eyre::Result<Vec<Kind>> {
    //     // let url = self.create_endpoint_url("r", su, endpoint, sort, sort_time, limit);

    //     let req = reqwest::Request::new(Method::GET, url);

    //     let data: ApiResponse = self.request_and_deserialize(req).await?;

    //     Ok(data.data.children)
    // }

    pub async fn subreddit_posts(
        &mut self,
        subreddit_name: &str,
        limit: Option<usize>,
    ) -> eyre::Result<Vec<Kind>> {
        let mut url = BASE_URL
            .join(&format!("/r/{subreddit_name}/new"))
            .wrap_err("could not create url")?;

        let query = [("limit", "100"), ("show", "all"), ("raw_json", "1")];

        url.query_pairs_mut().extend_pairs(query).finish();

        self.paginated(url, limit).await
    }

    pub async fn search_subreddits(
        &mut self,
        query: &str,
        limit: Option<u32>,
    ) -> eyre::Result<Vec<Subreddit>> {
        let mut url = BASE_URL
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
        let mut url = BASE_URL
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
        let mut url = BASE_URL
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
        let mut url = BASE_URL
            .join("/subreddits/popular")
            .wrap_err("could not create url")?;

        if let Some(n) = limit {
            url.query_pairs_mut()
                .extend_pairs([("limit", n.to_string().as_str()), ("raw_json", "1")]);
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

    // #[tokio::test]
    // async fn test_subreddit_posts_latest() -> Result<()> {
    //     let mut client = Reddit::new().await?;

    //     let result = client.subreddit_posts_latest("selfhosted").await?;

    //     dbg!(result);

    //     Ok(())
    // }

    #[tokio::test]
    async fn test_subreddit_about() -> Result<()> {
        let mut client = Reddit::new().await?;

        let sub = client.subreddit_about("rust").await?;

        dbg!(sub);

        Ok(())
    }
}
