use chrono::{NaiveTime, Utc};
use date_component::date_component;
use itertools::Itertools;

use crate::json::FlexibleIsoDate;

pub fn format_date(date: FlexibleIsoDate) -> String {
    date.0.format("%b %Y").to_string()
}

pub fn format_date_range(
    start: Option<&FlexibleIsoDate>,
    end: Option<&FlexibleIsoDate>,
) -> Option<String> {
    let start_date = match start.map(|date| date.0.clone()) {
        Some(start_date) => start_date,
        None => return None,
    };

    let present = Utc::now().date_naive();
    let end_date = end
        .map(|date| date.0.clone())
        .unwrap_or_else(|| present.clone());

    let formatted_range = if end_date == present {
        format!("{} – Present", start_date.format("%b %Y"))
    } else {
        format!(
            "{} – {}",
            start_date.format("%b %Y"),
            end_date.format("%b %Y"),
        )
    };

    let date_string = if start_date == end_date {
        formatted_range
    } else {
        let duration = date_component::calculate(
            &start_date.and_time(NaiveTime::default()).and_utc(),
            &end_date.and_time(NaiveTime::default()).and_utc(),
        );

        let formatted_duration = vec![
            pluralize(duration.year, "yr"),
            pluralize(duration.month, "mo"),
        ]
        .iter()
        .flatten()
        .join(" ");

        format!("{formatted_range} • {formatted_duration}")
    };

    Some(date_string)
}

fn pluralize(value: isize, name: &str) -> Option<String> {
    if value == 0 {
        return None;
    }
    Some(format!(
        "{value} {name}{}",
        if value == 1 { "" } else { "s" }
    ))
}
