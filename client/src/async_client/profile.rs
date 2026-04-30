use eyre::Context;

use crate::api::Kind;

impl super::Reddit {
    pub async fn following(&mut self, limit: Option<usize>) -> eyre::Result<Vec<Kind>> {
        let url = {
            let mut url = self
                .base_url
                .join("/subreddits/mine/subscriber")
                .wrap_err("could not create url")?;

            url.query_pairs_mut()
                .extend_pairs([("show", "all"), ("raw_json", "1")]);

            if let Some(limit) = limit {
                url.query_pairs_mut()
                    .append_pair("limit", &limit.to_string());
            }

            url
        };

        self.paginated(url, limit).await
    }
}

#[cfg(test)]
mod tests {

    use crate::async_client::Reddit;

    #[tokio::test]
    async fn test_following() -> eyre::Result<()> {
        tracing_subscriber::fmt().init();

        let mut client = Reddit::new().await?;

        let following = client.following(Some(110)).await?;

        dbg!(following);

        Ok(())
    }
}
