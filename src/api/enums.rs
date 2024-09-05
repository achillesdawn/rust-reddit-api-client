use std::fmt::Display;

use serde::Deserialize;

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