#![allow(unused)]

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ProfileResponse {
    pub data: Data,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub after: Value,
    pub before: Value,
    pub children: Vec<Child>,
    pub dist: i64,
    pub geo_filter: String,
    pub modhash: Value,
}

#[derive(Debug, Deserialize)]
pub struct Child {
    pub data: Post,
    pub kind: String,
}


#[derive(Debug, Deserialize)]
pub struct Post {
    pub all_awardings: Vec<Value>,
    pub allow_live_comments: bool,
    pub approved_at_utc: Value,
    pub approved_by: Value,
    pub archived: bool,
    pub author: String,
    pub author_flair_background_color: Option<String>,
    pub author_flair_css_class: Value,
    pub author_flair_richtext: Vec<Value>,
    pub author_flair_template_id: Option<String>,
    pub author_flair_text: Option<String>,
    pub author_flair_text_color: Option<String>,
    pub author_flair_type: String,
    pub author_fullname: String,
    pub author_is_blocked: bool,
    pub author_patreon_flair: bool,
    pub author_premium: bool,
    pub awarders: Vec<Value>,
    pub banned_at_utc: Value,
    pub banned_by: Value,
    pub can_gild: bool,
    pub can_mod_post: bool,
    pub category: Value,
    pub clicked: bool,
    pub content_categories: Value,
    pub contest_mode: bool,
    pub created: f64,
    pub created_utc: f64,
    pub discussion_type: Value,
    pub distinguished: Value,
    pub domain: String,
    pub downs: i64,
    pub edited: Value,
    pub gilded: i64,
    // pub gildings: Gildings,
    pub hidden: bool,
    pub hide_score: bool,
    pub id: String,
    pub is_created_from_ads_ui: bool,
    pub is_crosspostable: bool,
    pub is_meta: bool,
    pub is_original_content: bool,
    pub is_reddit_media_domain: bool,
    pub is_robot_indexable: bool,
    pub is_self: bool,
    pub is_video: bool,
    pub likes: Value,
    pub link_flair_background_color: String,
    pub link_flair_css_class: Option<String>,
    pub link_flair_richtext: Vec<Value>,
    pub link_flair_template_id: Option<String>,
    pub link_flair_text: Option<String>,
    pub link_flair_text_color: Option<String>,
    pub link_flair_type: String,
    pub locked: bool,
    pub media: Value,
    // pub media_embed: MediaEmbed,
    pub media_only: bool,
    pub mod_note: Value,
    pub mod_reason_by: Value,
    pub mod_reason_title: Value,
    pub mod_reports: Vec<Value>,
    pub name: String,
    pub no_follow: bool,
    pub num_comments: i64,
    pub num_crossposts: i64,
    pub num_reports: Value,
    pub over_18: bool,
    pub parent_whitelist_status: Value,
    pub permalink: String,
    pub pinned: bool,
    pub post_hint: Option<String>,
    pub preview: Option<Preview>,
    pub pwls: Value,
    pub quarantine: bool,
    pub removal_reason: Value,
    pub removed_by: Value,
    pub removed_by_category: Option<String>,
    pub report_reasons: Value,
    pub saved: bool,
    pub score: i64,
    pub secure_media: Value,
    pub secure_media_embed: SecureMediaEmbed,
    pub selftext: String,
    pub selftext_html: Value,
    pub send_replies: bool,
    pub spoiler: bool,
    pub stickied: bool,
    pub subreddit: String,
    pub subreddit_id: String,
    pub subreddit_name_prefixed: String,
    pub subreddit_subscribers: i64,
    pub subreddit_type: String,
    pub suggested_sort: Option<String>,
    pub thumbnail: String,
    pub thumbnail_height: Option<i64>,
    pub thumbnail_width: Option<i64>,
    pub title: String,
    pub top_awarded_type: Value,
    pub total_awards_received: i64,
    pub treatment_tags: Vec<Value>,
    pub ups: i64,
    pub upvote_ratio: f64,
    pub url: String,
    pub url_overridden_by_dest: Option<String>,
    pub user_reports: Vec<Value>,
    pub view_count: Value,
    pub visited: bool,
    pub whitelist_status: Value,
    pub wls: Value,
    pub gallery_data: Option<GalleryData>,
    pub is_gallery: Option<bool>,
    pub media_metadata: Option<HashMap<String, MediaMetaData>>,
}

// #[derive(Debug, Deserialize)]
// pub struct MediaEmbed {
// }

#[derive(Debug, Deserialize)]
pub struct Preview {
    pub enabled: bool,
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub id: String,
    pub resolutions: Vec<Resolution>,
    pub source: Source,
    pub variants: Variants,
}

#[derive(Debug, Deserialize)]
pub struct Resolution {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Debug, Deserialize)]
pub struct Variants {
    pub nsfw: Option<Nsfw>,
    pub obfuscated: Option<Obfuscated>,
}

#[derive(Debug, Deserialize)]
pub struct Nsfw {
    pub resolutions: Vec<Resolution>,
    pub source: Source,
}

#[derive(Debug, Deserialize)]
pub struct Obfuscated {
    pub resolutions: Vec<Resolution>,
    pub source: Source,
}

#[derive(Debug, Deserialize)]
pub struct SecureMediaEmbed {}

#[derive(Debug, Deserialize)]
pub struct GalleryData {
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub caption: String,
    pub id: i64,
    pub media_id: String,
    pub outbound_url: String,
}

#[derive(Debug, Deserialize)]
pub struct MediaMetaData {
    pub e: String,
    pub id: String,
    pub m: String,
    pub o: Vec<MediaPreview>,
    pub p: Vec<MediaPreview>,
    pub s: MediaPreview,
    pub status: String,
}
#[derive(Debug, Deserialize)]
pub struct MediaPreview {
    pub u: String,
    pub x: i64,
    pub y: i64,
}
