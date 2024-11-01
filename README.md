# Timewarrior Reports

A Rust application to parse and format time entries from Timewarrior tool output into a Markdown report.

## Project Structure

- **src/main.rs**: Reads input from stdin, processes JSON data, and outputs a markdown report.
- **src/lib.rs**: Declares modules used by the application.
- **src/json_parser.rs**: Contains functionality to parse JSON data into `TimeEntry` structs.
- **src/markdown.rs**: Responsible for formatting parsed data into a Markdown report.
- **src/grouper.rs**: Groups time entries by date and sums up durations.

## Usage

Build the project with Cargo:

```bash
cargo build --release
```

Then copy the file to timewarrior's extension directory in **~/.config/timewarrior/extensions/**.

## Module Overview

### `json_parser`

- Provides the `TimeEntry` struct for representing parsed data.
- `parse_json`: Parses JSON strings into a vector of `TimeEntry`.

### `markdown`

- `format_to_markdown`: Formats a time report as a Markdown document.

### `grouper`

- `group_entries`: Groups time entries by date and calculates durations.

## Testing

Tests are defined in `tests/test_json_parser.rs`.
Run the tests with:

```bash
cargo test
```

Make sure to modify and expand on the details as per your complete understanding of the project.
