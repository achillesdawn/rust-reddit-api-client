use eyre::Context;
use reqwest::Method;

use crate::api::{
    Post, RedditApiResponse,
    enums::{Endpoint, SortTime, SortType},
};

impl super::Reddit {
    pub async fn user_latest(
        &mut self,
        username: &str,
        endpoint: Endpoint,
        sort: SortType,
        sort_time: Option<SortTime>,
        limit: Option<u8>,
    ) -> eyre::Result<Vec<Post>> {
        let mut url = self
            .base_url
            .join(&format!("/user/{username}/{endpoint}"))
            .wrap_err("could not create url")?;

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

        let req = reqwest::Request::new(Method::GET, url);

        let data: RedditApiResponse<Post> = self.request_and_deserialize(req).await?;

        Ok(data.data.children.into_iter().map(|a| a.data).collect())
    }

    pub async fn user(&mut self, username: &str, limit: Option<usize>) -> eyre::Result<Vec<Post>> {
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

            let data: RedditApiResponse<Post> = self.request_and_deserialize(req).await?;

            posts.extend(data.data.children.into_iter().map(|child| child.data));

            if data.data.after.is_null() {
                break;
            } else if let Some(limit) = limit
                && posts.len() >= limit
            {
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

#[cfg(test)]
mod tests {
    use eyre::Result;

    use crate::async_client::Reddit;

    #[tokio::test]
    async fn test_user_posts_latest() -> Result<()> {
        tracing_subscriber::fmt::init();

        let mut client = Reddit::new().await?;

        let posts = client
            .user_latest(
                "e_o_raul",
                crate::async_client::user::Endpoint::Upvoted,
                crate::api::enums::SortType::Top,
                Some(crate::api::enums::SortTime::All),
                None,
            )
            .await?;

        dbg!(posts.len());

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_user_posts() -> eyre::Result<()> {
        tracing_subscriber::fmt::init();

        let mut client = Reddit::new().await?;

        let posts = client.user("e_o_raul", Some(100)).await?;

        let file = std::fs::File::create("example.json")?;

        serde_json::to_writer(file, &posts)?;

        Ok(())
    }
}
