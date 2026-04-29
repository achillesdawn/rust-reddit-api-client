use std::fmt::Display;

use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum DataType {
    t3,
    t5,
}

pub enum SortTime {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

impl Display for SortTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                SortTime::Hour => "hour",
                SortTime::Day => "day",
                SortTime::Week => "week",
                SortTime::Month => "month",
                SortTime::Year => "year",
                SortTime::All => "all",
            }
        )
    }
}

pub enum SortType {
    Hot,
    New,
    Top,
    Controlversial,
}

impl Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortType::Hot => write!(f, "hot"),
            SortType::New => write!(f, "new"),
            SortType::Top => write!(f, "top"),
            SortType::Controlversial => write!(f, "controversial"),
        }
    }
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
