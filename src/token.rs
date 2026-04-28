use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,

    #[serde(default = "timestamp_now", skip_deserializing)]
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
