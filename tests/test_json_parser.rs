use timewarrior_reporter::json_parser::parse_json;

#[test]
fn test_parse_json() {
    // Simulate input with config data followed by JSON data separated by an empty newline
    let input_data = r#"color: on
confirmation: on
debug: off

[{ "start": "2024-10-21T08:00:00Z", "end": "2024-10-21T09:15:00Z", "tags": ["Meeting"] }]"#;

    // Call parse_json with the simulated input data
    let entries = parse_json(input_data).expect("Failed to parse JSON data");

    // Verify that the parsed result matches the expected JSON data
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].tags.as_ref().unwrap()[0], "Meeting");
    assert_eq!(entries[0].start, "2024-10-21T08:00:00Z");
    assert_eq!(entries[0].end.as_ref().unwrap(), "2024-10-21T09:15:00Z");
}
