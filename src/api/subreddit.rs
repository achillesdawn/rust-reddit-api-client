use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Subreddit {
    pub display_name: String,
    pub title: String,
    pub subscribers: Option<u32>,
    pub display_name_prefixed: String,
    pub public_description: Option<String>,
    pub community_icon: Option<String>,
    pub icon_img: Option<String>,
    pub over18: Option<bool>,
    pub name: String,
    pub id: String,
    pub url: String,
    pub created_utc: f64,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
