use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedditError {
    #[error("environment variables not set")]
    NoEnvVariables,

    #[error("cannot reach api")]
    RequestError(#[from] Box<reqwest::Error>),

    #[error("unexpected response from api")]
    DeserializeError(#[from] serde_json::Error),

    #[error("unauthorized")]
    Unauthorized
}

pub enum HttpVerb {
    GET,
    POST,
}

impl Display for HttpVerb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpVerb::GET => "GET",
                HttpVerb::POST => "POST",
            }
        )
    }
}