use chrono::{DateTime, FixedOffset};

pub struct Post {
    title: Option<String>,
    description: String,
}

pub struct Dates {
    pub created: DateTime<FixedOffset>,
    pub published: DateTime<FixedOffset>,
    pub updated: Option<DateTime<FixedOffset>>,
}

