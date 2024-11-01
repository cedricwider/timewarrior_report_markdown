use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TimeEntry {
    pub start: String,
    pub end: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub fn parse_compact_datetime(date_str: &str) -> DateTime<FixedOffset> {
    let naive = NaiveDateTime::parse_from_str(date_str, "%Y%m%dT%H%M%SZ")
        .expect("Failed to parse compact date-time format");
    DateTime::from_naive_utc_and_offset(naive, FixedOffset::east_opt(0).expect("Invalid offset"))
}

pub fn parse_json(data: &str) -> Result<Vec<TimeEntry>, serde_json::Error> {
    // Split the input by empty newlines and take the second part
    let json_data = data
        .split("\n\n") // Split by empty newline
        .nth(1) // Take the JSON portion (second section)
        .unwrap_or(""); // Use an empty string if there's no JSON data

    // Parse JSON
    serde_json::from_str(json_data)
}
