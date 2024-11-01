use crate::json_parser::{parse_compact_datetime, TimeEntry};
use chrono::{Duration, FixedOffset};
use std::collections::HashMap;

pub fn group_entries(entries: Vec<TimeEntry>) -> HashMap<String, HashMap<String, Duration>> {
    let mut report: HashMap<String, HashMap<String, Duration>> = HashMap::new();

    for entry in entries {
        let start_time = parse_compact_datetime(&entry.start);
        let end_time = match entry.end {
            Some(end) => parse_compact_datetime(&end),
            None => chrono::Local::now()
                .with_timezone(&FixedOffset::east_opt(0).expect("Invalid offset")),
        };

        let duration = end_time.signed_duration_since(start_time);
        let date_str = start_time.format("%Y-%m-%d").to_string();
        let main_tag = entry
            .tags
            .as_ref()
            .and_then(|tags| tags.first())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "No Tag".to_string());

        report
            .entry(date_str)
            .or_default()
            .entry(main_tag)
            .and_modify(|e| *e += duration)
            .or_insert(duration);
    }

    report
}
