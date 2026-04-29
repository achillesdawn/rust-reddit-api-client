use std::fmt::Display;

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
