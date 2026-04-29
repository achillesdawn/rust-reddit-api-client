use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Subreddit {
    pub display_name: String,
    pub title: String,
    pub subscribers: Option<u64>,
    pub display_name_prefixed: String,
    pub public_description: Option<String>,
    pub public_description_html: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub community_icon: Option<String>,
    pub icon_img: Option<String>,
    pub over18: Option<bool>,
    pub name: String,
    pub id: String,
    pub url: String,
    pub created_utc: f64,
    pub created: f64,
    pub subreddit_type: String,
    pub submission_type: Option<String>,
    pub user_is_subscriber: Option<bool>,
    pub user_is_moderator: Option<bool>,
    pub user_is_banned: Option<bool>,
    pub user_is_muted: Option<bool>,
    pub header_img: Option<String>,
    pub banner_background_color: Option<String>,
    pub submit_text: Option<String>,
    pub submit_text_html: Option<String>,
    pub accounts_active: Option<u64>,
    pub active_user_count: Option<u64>,

    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
