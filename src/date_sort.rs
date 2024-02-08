use std::cmp::Ordering;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, Utc};

#[derive(Clone, PartialEq, Eq, derive_more::From)]
pub enum DateSort {
    Now,

    #[from]
    Past(DateTime<Utc>),
}

impl Ord for DateSort {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (DateSort::Now, DateSort::Now) => Ordering::Equal,
            (DateSort::Now, DateSort::Past(_)) => Ordering::Greater,
            (DateSort::Past(_), DateSort::Now) => Ordering::Less,
            (DateSort::Past(a), DateSort::Past(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for DateSort {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<NaiveDate> for DateSort {
    fn from(value: NaiveDate) -> Self {
        value
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .into()
    }
}
impl From<DateTime<FixedOffset>> for DateSort {
    fn from(value: DateTime<FixedOffset>) -> Self {
        value.to_utc().into()
    }
}
