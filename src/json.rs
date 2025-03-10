use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(::serde::Serialize, Clone, Debug)]
pub struct FlexibleIsoDate(pub NaiveDate);

impl<'de> Deserialize<'de> for FlexibleIsoDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        parse_flexible_date(s)
            .map(FlexibleIsoDate)
            .ok_or_else(|| serde::de::Error::custom("Invalid ISO 8601 date"))
    }
}

fn parse_flexible_date(date_str: &str) -> Option<NaiveDate> {
    let parts: Vec<&str> = date_str.split('-').collect();

    let year = i32::from_str(parts[0]).ok()?;
    let month = if parts.len() > 1 {
        u32::from_str(parts[1]).ok()?
    } else {
        1
    };
    let day = if parts.len() > 2 {
        u32::from_str(parts[2]).ok()?
    } else {
        1
    };

    NaiveDate::from_ymd_opt(year, month, day)
}
