use base64::prelude::*;
use eyre::Context;
use tracing::debug;

use crate::token::Token;

impl super::Reddit {
    fn encode_authorization(client_id: String, client_secret: String) -> String {
        let encoded = BASE64_STANDARD_NO_PAD.encode(format!("{}:{}", client_id, client_secret));
        let encoded_auth = format!("Basic {}", encoded);
        encoded_auth
    }

    pub async fn authorize(&mut self) -> eyre::Result<()> {
        let (Ok(client_id), Ok(client_secret), Ok(username), Ok(password)) = (
            std::env::var("REDDIT_CLIENT_ID"),
            std::env::var("REDDIT_CLIENT_SECRET"),
            std::env::var("REDDIT_USERNAME"),
            std::env::var("REDDIT_PASSWORD"),
        ) else {
            return Err(eyre::eyre!(
                "could not retrieve auth environment variables: protip read the .env file"
            ));
        };

        let encoded_auth = super::Reddit::encode_authorization(client_id, client_secret);

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
            .wrap_err("could not send auth request")?
            .error_for_status()
            .wrap_err("response status error")?;

        let bytes = res.bytes().await.unwrap();

        let token: Token =
            serde_json::from_slice(&bytes).wrap_err("could not deserialize byte response")?;

        debug!(?token);

        let mut default_headers = reqwest::header::HeaderMap::from_iter([(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token.access_token))
                .unwrap(),
        )]);

        let client = reqwest::ClientBuilder::new()
            .user_agent("Rust: trends v0.1.0 by u/molivo10")
            .default_headers(default_headers)
            .build()
            .unwrap();

        self.client = client;
        self.token = token;

        Ok(())
    }
}
