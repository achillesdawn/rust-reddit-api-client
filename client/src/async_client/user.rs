use reqwest::Method;
use url::Url;

use crate::api::{
    RedditApiResponse, UserItem,
    enums::{Endpoint, SortTime, SortType},
};

impl super::Reddit {
    fn create_user_url(
        &self,
        username: &str,
        endpoint: Endpoint,
        sort: SortType,
        sort_time: Option<SortTime>,
        limit: Option<usize>,
    ) -> Url {
        let mut url = self
            .base_url
            .join(&format!("/user/{username}/{endpoint}"))
            .expect("could not create url");

        let limit = limit.map(|i| i.to_string()).unwrap_or("100".to_owned());

        if let Some(sort) = sort_time {
            url.query_pairs_mut().append_pair("t", &sort.to_string());
        }

        url.query_pairs_mut()
            .extend_pairs([
                ("context", "2"),
                ("show", "given"),
                ("sort", &sort.to_string()),
                ("type", "links"),
                ("limit", limit.as_str()),
                ("raw_json", "1"),
            ])
            .finish();

        url
    }

    pub async fn user_latest(
        &mut self,
        username: &str,
        endpoint: Endpoint,
        sort: SortType,
        sort_time: Option<SortTime>,
        limit: Option<usize>,
    ) -> eyre::Result<Vec<UserItem>> {
        let url = self.create_user_url(username, endpoint, sort, sort_time, limit);

        let req = reqwest::Request::new(Method::GET, url);

        let data: RedditApiResponse<UserItem> = self.request_and_deserialize(req).await?;

        Ok(data.data.children.into_iter().map(|a| a.data).collect())
    }

    pub async fn user(
        &mut self,
        username: &str,
        endpoint: Endpoint,
        sort: SortType,
        sort_time: Option<SortTime>,
        limit: Option<usize>,
    ) -> eyre::Result<Vec<UserItem>> {
        let url = self.create_user_url(username, endpoint, sort, sort_time, limit);

        let mut posts = Vec::new();
        let mut after: Option<String> = None;

        loop {
            let mut request_url = url.clone();

            if let Some(after) = after {
                request_url.query_pairs_mut().append_pair("after", &after);
            }

            let req = reqwest::Request::new(Method::GET, url.clone());

            let data: RedditApiResponse<UserItem> = self.request_and_deserialize(req).await?;

            posts.extend(data.data.children.into_iter().map(|child| child.data));

            if data.data.after.is_null() {
                break;
            } else if let Some(limit) = limit
                && posts.len() >= limit
            {
                break;
            } else {
                after = Some(data.data.after.as_str().unwrap().to_owned());
            }
        }

        Ok(posts)
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use crate::{api::enums::SortTime, async_client::Reddit};

    #[tokio::test]
    async fn test_user_posts_latest() -> Result<()> {
        tracing_subscriber::fmt::init();

        let mut client = Reddit::new().await?;

        let items = client
            .user_latest(
                "e_o_raul",
                crate::api::enums::Endpoint::Comments,
                crate::api::enums::SortType::Top,
                Some(crate::api::enums::SortTime::All),
                None,
            )
            .await?;

        for item in items {
            dbg!(item);
        }

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_user_posts() -> eyre::Result<()> {
        tracing_subscriber::fmt::init();

        let mut client = Reddit::new().await?;

        let items = client
            .user(
                "e_o_raul",
                crate::api::enums::Endpoint::Upvoted,
                crate::api::enums::SortType::Top,
                Some(SortTime::Month),
                Some(100),
            )
            .await?;

        let file = std::fs::File::create("example.json")?;

        serde_json::to_writer(file, &items)?;

        Ok(())
    }
}
