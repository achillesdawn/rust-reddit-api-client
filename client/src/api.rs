use serde::Deserialize;
use serde_json::Value;

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
    pub modhash: String,
}

#[derive(Debug, Deserialize)]
pub struct Child<T> {
    pub data: T,
    pub kind: DataType,
}

#[derive(Debug, Deserialize)]
pub struct RedditApiResponseT {
    pub data: PagingDataT,
    pub kind: EndpointType,
}

#[derive(Debug, Deserialize)]
pub struct PagingDataT {
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<Kind>,
    pub dist: u32,
    pub geo_filter: String,
    pub modhash: Option<String>,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "data")]
pub enum Kind {
    #[serde(rename = "t3")]
    Post(Box<Post>),

    #[serde(rename = "t1")]
    Comment(Box<Comment>),
}
