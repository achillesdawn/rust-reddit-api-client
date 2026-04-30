use serde::Deserialize;
use serde_json::Value;

mod comment;
pub mod enums;
mod post;
mod profile;
mod subreddit;

pub use comment::CommentData;
pub use enums::DataType;
pub use post::Post;
pub use profile::Profile;
pub use subreddit::Subreddit;

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum UserItem {
    Post(Box<Post>),
    Comment(Box<CommentData>),
}

#[derive(Debug, Deserialize)]
pub enum EndpointType {
    Listing,
}

#[derive(Debug, Deserialize)]
pub struct RedditApiResponse<T> {
    pub data: PagingData<T>,
    pub kind: EndpointType,
}

#[derive(Debug, Deserialize)]
pub struct PagingData<T> {
    pub after: Value,
    pub before: Value,
    pub children: Vec<Child<T>>,
    pub dist: u32,
    pub geo_filter: String,
    pub modhash: Value,
}

#[derive(Debug, Deserialize)]
pub struct Child<T> {
    pub data: T,
    pub kind: DataType,
}
