use serde::{Deserialize, Serialize};

/// 88x31 button representation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Button88x31 {
    pub img: String,
    pub title: Option<String>,
    pub alt: Option<String>,
    pub href: Option<String>,
    pub onclick: Option<String>,
}
