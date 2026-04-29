use std::fmt::Display;

use serde::Deserialize;

#[allow(dead_code)]
enum RedditTime {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

impl Display for RedditTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                RedditTime::Hour => "hour",
                RedditTime::Day => "day",
                RedditTime::Week => "week",
                RedditTime::Month => "month",
                RedditTime::Year => "year",
                RedditTime::All => "all",
            }
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum DataType {
    t3,
    t5,
}
pub enum Endpoint {
    Overview,
    Submitted,
    Comments,
    Upvoted,
    Downvoted,
    Hidden,
    Saved,
    Gilded,
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endpoint::Overview => write!(f, "overview"),
            Endpoint::Submitted => write!(f, "submitted"),
            Endpoint::Comments => write!(f, "comments"),
            Endpoint::Upvoted => write!(f, "upvoted"),
            Endpoint::Downvoted => write!(f, "downvoted"),
            Endpoint::Hidden => write!(f, "hidden"),
            Endpoint::Saved => write!(f, "saved"),
            Endpoint::Gilded => write!(f, "gilded"),
        }
    }
}

pub enum Sort {
    Hot,
    New,
    Top,
    Controlversial,
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sort::Hot => write!(f, "hot"),
            Sort::New => write!(f, "new"),
            Sort::Top => write!(f, "top"),
            Sort::Controlversial => write!(f, "controversial"),
        }
    }
}

pub enum TopTime {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

impl Display for TopTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopTime::Hour => write!(f, "hour"),
            TopTime::Day => write!(f, "day"),
            TopTime::Week => write!(f, "week"),
            TopTime::Month => write!(f, "month"),
            TopTime::Year => write!(f, "year"),
            TopTime::All => write!(f, "all"),
        }
    }
}
