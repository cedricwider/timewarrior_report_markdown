use std::io::{self, Read};
use timewarrior_reporter::grouper::group_entries;
use timewarrior_reporter::json_parser::parse_json;
use timewarrior_reporter::markdown::format_to_markdown;

fn main() {
    // Read data from stdin
    let mut data = String::new();
    io::stdin()
        .read_to_string(&mut data)
        .expect("Failed to read from stdin");

    // Check for empty or non-JSON input
    if data.trim().is_empty() {
        eprintln!("No valid JSON data received from Timewarrior. Please ensure there are time entries for the specified period.");
        return;
    }

    // Parse JSON and process
    let entries = match parse_json(&data) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to parse JSON data: {}", err);
            return;
        }
    };

    let report = group_entries(entries);
    let markdown = format_to_markdown(report);

    // Output the markdown report
    println!("{}", markdown);
}
