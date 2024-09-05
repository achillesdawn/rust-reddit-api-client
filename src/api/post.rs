use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;


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
    pub downs: u32,
    pub edited: Value,
    pub gilded: u32,
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
    pub link_flair_background_color: Option<String>,
    pub link_flair_css_class: Option<String>,
    pub link_flair_richtext: Vec<Value>,
    pub link_flair_template_id: Option<String>,
    pub link_flair_text: Option<String>,
    pub link_flair_text_color: Option<String>,
    pub link_flair_type: String,
    pub locked: bool,
    pub media: Option<Media>,
    // pub media_embed: MediaEmbed,
    pub media_metadata: Option<HashMap<String, MediaMetaData>>,
    pub media_only: bool,
    pub mod_note: Value,
    pub mod_reason_by: Value,
    pub mod_reason_title: Value,
    pub mod_reports: Vec<Value>,
    pub name: String,
    pub no_follow: bool,
    pub num_comments: u32,
    pub num_crossposts: u32,
    pub num_reports: Value,
    pub over_18: bool,
    pub parent_whitelist_status: Option<String>,
    pub permalink: String,
    pub pinned: bool,
    pub post_hint: Option<String>,
    pub preview: Option<Preview>,
    pub pwls: Option<u64>,
    pub quarantine: bool,
    pub removal_reason: Value,
    pub removed_by: Value,
    pub removed_by_category: Option<String>,
    pub report_reasons: Value,
    pub saved: bool,
    pub score: u64,
    pub secure_media: Option<Media>,
    pub secure_media_embed: SecureMediaEmbed,
    pub selftext: String,
    pub selftext_html: Value,
    pub send_replies: bool,
    pub spoiler: bool,
    pub stickied: bool,
    pub subreddit: String,
    pub subreddit_id: String,
    pub subreddit_name_prefixed: String,
    pub subreddit_subscribers: u32,
    pub subreddit_type: String,
    pub suggested_sort: Option<String>,
    pub thumbnail: String,
    pub thumbnail_height: Option<u32>,
    pub thumbnail_width: Option<u32>,
    pub title: String,
    pub top_awarded_type: Value,
    pub total_awards_received: u32,
    pub treatment_tags: Vec<Value>,
    pub ups: u32,
    pub upvote_ratio: f64,
    pub url: String,
    pub url_overridden_by_dest: Option<String>,
    pub user_reports: Vec<Value>,
    pub view_count: Value,
    pub visited: bool,
    pub whitelist_status: Option<String>,
    pub wls: Value,
    pub gallery_data: Option<GalleryData>,
    pub is_gallery: Option<bool>,
}

// #[derive(Debug, Deserialize)]
// pub struct MediaEmbed {
// }

#[derive(Debug, Deserialize)]
pub struct Media {
    reddit_video: Option<RedditVideo>,
}

#[derive(Debug, Deserialize)]
pub struct RedditVideo {
    bitrate_kbps: u32,
    dash_url: String,
    duration: u32,
    fallback_url: String,
    has_audio: bool,
    height: u16,
    width: u16,
    hls_url: String,
    is_gif: bool,
    scrubber_media_url: String,
    transcoding_status: String,
}

#[derive(Debug, Deserialize)]
pub struct Preview {
    pub enabled: bool,
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub id: String,
    pub resolutions: Vec<Resolution>,
    pub source: Resolution,
    pub variants: Option<Variants>,
}

#[derive(Debug, Deserialize)]
pub struct Resolution {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Debug, Deserialize)]
pub struct Variants {
    pub nsfw: Option<Nsfw>,
    pub obfuscated: Option<Obfuscated>,
}

#[derive(Debug, Deserialize)]
pub struct Nsfw {
    pub resolutions: Vec<Resolution>,
    pub source: Resolution,
}

#[derive(Debug, Deserialize)]
pub struct Obfuscated {
    pub resolutions: Vec<Resolution>,
    pub source: Resolution,
}

#[derive(Debug, Deserialize)]
pub struct SecureMediaEmbed {}

#[derive(Debug, Deserialize)]
pub struct GalleryData {
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub caption: Option<String>,
    pub id: u32,
    pub media_id: String,
    pub outbound_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MediaMetaData {
    pub e: String,
    pub id: String,
    pub m: Option<String>,
    pub o: Option<Vec<MediaPreview>>,
    pub p: Option<Vec<MediaPreview>>,
    pub s: Option<MediaPreview>,
    pub status: String,
}
#[derive(Debug, Deserialize)]
pub struct MediaPreview {
    pub u: Option<String>,
    pub x: u32,
    pub y: u32,
}
