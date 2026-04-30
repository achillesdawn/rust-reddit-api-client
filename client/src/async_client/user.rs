use reqwest::Method;

use crate::api::{
    ApiResponse, BASE_URL, Kind,
    enums::{Endpoint, SortTime, SortType},
};

fn create_endpoint_url(
    username: &str,
    endpoint: Endpoint,
    sort: SortType,
    sort_time: Option<SortTime>,
    limit: Option<usize>,
) -> url::Url {
    let mut url = BASE_URL
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

impl super::Reddit {
    pub async fn user_latest(
        &mut self,
        username: &str,
        endpoint: Endpoint,
        sort: SortType,
        sort_time: Option<SortTime>,
        limit: Option<usize>,
    ) -> eyre::Result<ApiResponse> {
        let url = create_endpoint_url(username, endpoint, sort, sort_time, limit);

        let req = reqwest::Request::new(Method::GET, url);

        self.request_and_deserialize(req).await
    }

    pub async fn user(
        &mut self,
        username: &str,
        endpoint: Endpoint,
        sort: SortType,
        sort_time: Option<SortTime>,
        limit: Option<usize>,
    ) -> eyre::Result<Vec<Kind>> {
        let url = create_endpoint_url(username, endpoint, sort, sort_time, limit);

        self.paginated(url, limit).await
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use crate::{
        api::enums::{Endpoint, SortTime, SortType},
        async_client::Reddit,
    };

    #[tokio::test]
    async fn test_user_posts_latest() -> Result<()> {
        tracing_subscriber::fmt::init();

        let mut client = Reddit::new().await?;

        let items = client
            .user_latest(
                "e_o_raul",
                Endpoint::Overview,
                SortType::New,
                Some(SortTime::All),
                Some(10),
            )
            .await?;

        dbg!(items);
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
                Endpoint::Comments,
                SortType::New,
                Some(SortTime::Month),
                Some(100),
            )
            .await?;

        let file = std::fs::File::create("example.json")?;

        serde_json::to_writer(file, &items)?;

        Ok(())
    }
}
