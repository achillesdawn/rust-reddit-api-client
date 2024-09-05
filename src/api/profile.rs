use serde::Deserialize;
use serde::Serialize;

use super::DataType;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub accept_followers: bool,
    // pub accounts_active: Value,
    pub accounts_active_is_fuzzed: bool,
    // pub active_user_count: Value,
    pub advertiser_category: String,
    pub all_original_content: bool,
    pub allow_discovery: bool,
    pub allow_galleries: bool,
    pub allow_images: bool,
    pub allow_polls: bool,
    pub allow_prediction_contributors: bool,
    pub allow_predictions: bool,
    pub allow_predictions_tournament: bool,
    pub allow_talks: bool,
    pub allow_videogifs: bool,
    pub allow_videos: bool,
    pub allowed_media_in_comments: Vec<String>,
    pub banner_background_color: String,
    pub banner_background_image: String,
    pub banner_img: String,
    pub banner_size: Option<[u16; 2]>,
    pub can_assign_link_flair: bool,
    pub can_assign_user_flair: bool,
    pub collapse_deleted_comments: bool,
    pub comment_contribution_settings: CommentContributionSettings,
    pub comment_score_hide_mins: i64,
    pub community_icon: String,
    pub community_reviewed: bool,
    pub created: f64,
    pub created_utc: f64,
    pub description: String,
    pub description_html: Option<String>,
    pub disable_contributor_requests: bool,
    pub display_name: String,
    pub display_name_prefixed: String,
    // pub emojis_custom_size: Value,
    pub emojis_enabled: bool,
    pub free_form_reports: bool,
    pub has_menu_widget: bool,
    pub header_img: Option<String>,
    pub header_size: Option<[u16; 2]>,
    pub header_title: String,
    pub hide_ads: bool,
    pub icon_img: String,
    pub icon_size: Option<[u16; 2]>,
    pub id: String,
    pub is_crosspostable_subreddit: Option<bool>,
    pub is_default_banner: Option<bool>,
    #[serde(default)]
    pub is_default_icon: Vec<bool>,
    // pub is_enrolled_in_new_modmail: Value,
    pub key_color: String,
    pub lang: String,
    pub link_flair_enabled: bool,
    pub link_flair_position: String,
    pub mobile_banner_image: String,
    pub name: String,
    pub notification_level: Option<String>,
    pub original_content_tag_enabled: bool,
    pub over18: bool,
    pub prediction_leaderboard_entry_type: i64,
    pub primary_color: String,
    pub public_description: String,
    pub public_description_html: Option<String>,
    pub public_traffic: bool,
    pub quarantine: bool,
    pub restrict_commenting: bool,
    pub restrict_posting: bool,
    pub should_archive_posts: bool,
    pub should_show_media_in_comments_setting: bool,
    pub show_media: bool,
    pub show_media_preview: bool,
    pub spoilers_enabled: bool,
    pub submission_type: String,
    pub submit_link_label: String,
    pub submit_text: String,
    pub submit_text_html: Option<String>,
    pub submit_text_label: String,
    pub subreddit_type: String,
    pub subscribers: i64,
    pub suggested_comment_sort: Option<String>,
    pub title: String,
    pub url: String,
    // pub user_can_flair_in_sr: Value,
    // pub user_flair_background_color: Value,
    // pub user_flair_css_class: Value,
    pub user_flair_enabled_in_sr: bool,
    pub user_flair_position: String,
    // pub user_flair_richtext: Vec<Value>,
    // pub user_flair_template_id: Value,
    // pub user_flair_text: Value,
    // pub user_flair_text_color: Value,
    pub user_flair_type: String,
    pub user_has_favorited: bool,
    pub user_is_banned: bool,
    pub user_is_contributor: bool,
    pub user_is_moderator: bool,
    pub user_is_muted: bool,
    pub user_is_subscriber: bool,
    // pub user_sr_flair_enabled: Value,
    pub user_sr_theme_enabled: bool,
    pub whitelist_status: Option<String>,
    pub wiki_enabled: Option<bool>,
    pub wls: Option<i64>,
    pub videostream_links_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CommentContributionSettings {
    #[serde(default)]
    pub allowed_media_types: Option<Vec<String>>,
}
