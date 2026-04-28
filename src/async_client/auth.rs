use base64::prelude::*;
use eyre::Context;

use crate::token::Token;

impl super::Reddit {
    pub fn auth_token_expired(&self) -> bool {
        self.token.is_expired()
    }

    fn encode_authorization(client_id: String, client_secret: String) -> String {
        let encoded = BASE64_STANDARD_NO_PAD.encode(format!("{}:{}", client_id, client_secret));

        let encoded_auth = format!("Basic {}", encoded);

        encoded_auth
    }

    pub async fn authenticate() -> eyre::Result<Token> {
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

        let res = reqwest::Client::new()
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

        let bytes = res
            .bytes()
            .await
            .wrap_err("could not read auth respose bytes")?;

        let token: Token =
            serde_json::from_slice(&bytes).wrap_err("could not deserialize byte response")?;

        Ok(token)
    }

    pub async fn re_authenticate(&mut self) -> eyre::Result<()> {
        let new_token = super::Reddit::authenticate().await?;

        let client = super::Reddit::build_client(new_token.clone());

        self.client = client;
        self.token = new_token;

        Ok(())
    }
}
