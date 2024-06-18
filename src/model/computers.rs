use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Computer {
    pub name: String,
    pub hostname: Option<String>,
    pub slug: String,
    pub date: ComputerDates,
    pub status: Status,
    pub specs: Specs,
    pub decom_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    #[serde(rename = "in-use")]
    InUse,
    #[serde(rename = "decomissioning")]
    Decomissioning,
    #[serde(rename = "decomissioned")]
    Decomissioned,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComputerDates {
    pub acquired: NaiveDate,
    pub decomissioned: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Specs {
    pub r#type: String,
    pub model: String,
    pub cpu: String,
    pub ram: Ram,
    pub storage: Vec<Storage>,
    pub gpu: Option<String>,
    pub motherboard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Storage {
    pub r#type: String,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ram {
    pub size: String,
    pub gen: String,
    pub speed: Option<String>,
}
