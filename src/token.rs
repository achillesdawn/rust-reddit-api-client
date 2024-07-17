use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
