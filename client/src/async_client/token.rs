use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,

    #[serde(default = "timestamp_now", skip_deserializing, skip_serializing)]
    token_valid_since: DateTime<Utc>,
}

fn timestamp_now() -> DateTime<Utc> {
    Utc::now()
}

#[allow(unused)]
impl Token {
    pub fn new() -> Self {
        Token {
            access_token: "".to_owned(),
            expires_in: 0,
            scope: "".to_owned(),
            token_type: "".to_owned(),
            token_valid_since: Utc::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let elapsed = now
            .signed_duration_since(self.token_valid_since)
            .num_seconds();

        if elapsed > self.expires_in {
            return true;
        }

        false
    }

    pub fn cache(&self) -> eyre::Result<()> {
        tracing::debug!("writing token to cache");

        let file = std::fs::File::open("cached.token")?;

        serde_json::to_writer(file, &self)?;

        Ok(())
    }

    pub fn read_cache() -> eyre::Result<Self> {
        let file = std::fs::File::open("cached.token")?;

        let token: Token = serde_json::from_reader(file)?;

        tracing::debug!("read token from cache");

        Ok(token)
    }
}

impl From<Token> for HeaderMap {
    fn from(val: Token) -> Self {
        HeaderMap::from_iter([(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", val.access_token))
                .unwrap(),
        )])
    }
}

impl Default for Token {
    fn default() -> Self {
        Self::new()
    }
}
