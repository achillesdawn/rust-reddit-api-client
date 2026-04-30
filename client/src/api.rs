use serde::Deserialize;
use std::sync::LazyLock;
use url::Url;

pub static BASE_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://oauth.reddit.com").expect("invalid base url"));

mod comment;
pub mod enums;
mod post;
mod profile;
mod subreddit;

pub use comment::Comment;
pub use enums::DataType;
pub use post::Post;
pub use profile::Profile;
pub use subreddit::Subreddit;

#[derive(Debug, Deserialize)]
pub struct RedditApiResponse<T> {
    pub data: PagingData<T>,
    pub kind: EndpointType,
}

#[derive(Debug, Deserialize)]
pub enum EndpointType {
    Listing,
}

#[derive(Debug, Deserialize)]
pub struct PagingData<T> {
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<Child<T>>,
    pub dist: u32,
    pub geo_filter: String,
    // pub modhash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Child<T> {
    pub data: T,
    pub kind: DataType,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: PagedResponse,
    pub kind: EndpointType,
}

#[derive(Debug, Deserialize)]
pub struct PagedResponse {
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<Kind>,
    pub dist: u32,
    pub geo_filter: String,
    // pub modhash: Option<String>,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "data")]
pub enum Kind {
    #[serde(rename = "t1")]
    Comment(Box<Comment>),

    #[serde(rename = "t3")]
    Post(Box<Post>),

    #[serde(rename = "t5")]
    Profile(Box<Profile>),
}
