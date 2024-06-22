use std::{cmp::Ordering, fmt::Display};

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

impl Computer {
    pub fn href(&self) -> String {
        format!("/computers/{}", self.slug)
    }

    pub fn display_ordering(&self, other: &Self) -> Ordering {
        match (&self.date.decomissioned, &other.date.decomissioned) {
            // both are decommed -- sort by decom date
            (Some(a), Some(b)) => a.cmp(b),

            // both are acquired -- sort by acquire date
            (None, None) => self.date.acquired.cmp(&other.date.acquired),

            // all decommed come before all acquired
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
        }
    }
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

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::InUse => write!(f, "In use"),
            Status::Decomissioning => write!(f, "Decomissioning"),
            Status::Decomissioned => write!(f, "Decomissioned"),
        }
    }
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
