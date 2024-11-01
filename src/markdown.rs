// src/markdown.rs
use chrono::Duration;
use std::collections::HashMap;

pub fn format_to_markdown(report: HashMap<String, HashMap<String, Duration>>) -> String {
    let mut output = String::new();
    output.push_str(&format!(
        "# TimeReport {} - {}\n",
        chrono::Local::now().format("%Y-%m-%d"),
        chrono::Local::now().format("%Y-%m-%d")
    ));

    for (date, tasks) in report {
        output.push_str(&format!(
            "\n## {}\n\n| Task | Duration |\n| ---- | -------- |\n",
            date
        ));
        for (task, duration) in tasks {
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;
            output.push_str(&format!("| {} | {}h {}m |\n", task, hours, minutes));
        }
    }

    output
}
