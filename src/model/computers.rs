use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Computers {
    pub name: String,
    pub model: String,
    pub start_year: usize,
    pub end_year: Option<usize>,
    pub decom_reason: Option<String>,
}
