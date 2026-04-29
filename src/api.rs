use serde::Deserialize;
use serde_json::Value;

mod enums;
mod post;
mod profile;
mod subreddit;

pub use enums::DataType;
pub use post::Post;
pub use profile::Profile;
pub use subreddit::Subreddit;

#[derive(Debug, Deserialize)]
pub enum EndpointType {
    Listing,
}

#[derive(Debug, Deserialize)]
pub struct RedditApiResonse<T> {
    pub data: PagingData<T>,
    pub kind: EndpointType,
}

#[derive(Debug, Deserialize)]
pub struct RedditSingleResponse<T> {
    pub data: T,
    pub kind: DataType,
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
