use serde::{Deserialize, Serialize};

/// Webring representation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Webring {
    pub prev: String,
    pub next: String,
    pub html: String,
    pub pending: bool,
}
