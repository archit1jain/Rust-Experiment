use crate::error::ParseError;
use crate::models::LogEntry;

// This function takes one raw line from the log file and turns it into
// structured information that the rest of the application can understand.
// For example, it separates the date, HTTP method, endpoint, status code,
// and response time from the original line.
//
/// Parses a single line of log text into a `LogEntry`.
///
/// Takes a borrowed string slice `&str` instead of taking ownership,
/// allowing the caller to retain the original String if they need to.
// A `Result<T, E>` in Rust represents an operation that can either succeed or fail.
// - `Ok(LogEntry)` means the parse succeeded and holds the resulting entry.
// - `Err(ParseError)` means it failed and holds the reason why.
pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    // We use `split_whitespace()` which creates an Iterator yielding string slices (`&str`).
    // An iterator is an object that yields values one by one instead of loading everything into memory at once.
    // `mut` is required because pulling the next item out of the iterator modifies its internal state.
    let mut parts = line.split_whitespace();

    // `.next()` asks the iterator for the next piece of data.
    // It returns an `Option`: `Some(value)` if a value is present, or `None` if the iterator is empty.
    // `.ok_or()` converts the `Option` into a `Result`. If it's `None`, it becomes `Err(ParseError::MissingFields)`.
    // The `?` (question mark) operator checks the `Result`.
    // If the `Result` is `Ok`, it unwraps the value. If it's `Err`, it immediately returns that error from the `parse_line` function.
    // Finally, `.to_string()` allocates memory and copies the borrowed text into an owned `String`.
    let timestamp = parts.next().ok_or(ParseError::MissingFields)?.to_string();
    let method = parts.next().ok_or(ParseError::MissingFields)?.to_string();
    let endpoint = parts.next().ok_or(ParseError::MissingFields)?.to_string();

    // Parse the status code.
    // The `parse::<u16>()` method on `&str` returns a `Result<u16, ParseIntError>`.
    // Thanks to the `From` trait implementation in `error.rs`, the `?` operator automatically
    // converts `ParseIntError` into `ParseError::InvalidNumber` before returning it.
    let status_str = parts.next().ok_or(ParseError::MissingFields)?;
    let status: u16 = status_str.parse()?;

    // Parse the response time (latency). It expects a format like "123ms".
    let latency_str = parts.next().ok_or(ParseError::MissingFields)?;

    // We check if the string ends with "ms" using the `ends_with` method.
    // If not, we explicitly return an `Err` wrapped around our custom error type.
    if !latency_str.ends_with("ms") {
        return Err(ParseError::InvalidLatencyFormat);
    }

    // Slice off the last two characters ("ms"). `len()` gives byte length.
    // `&latency_str[..latency_str.len() - 2]` creates a new borrowed view (`&str`) into the existing string, ignoring the "ms".
    // We then parse the remaining string slice.
    let ms_part = &latency_str[..latency_str.len() - 2];
    let response_time_ms: u64 = ms_part.parse()?;

    // If we've made it this far, parsing was successful.
    // We construct the `LogEntry` struct and return it wrapped in `Ok`.
    Ok(LogEntry {
        timestamp,
        method,
        endpoint,
        status,
        response_time_ms,
    })
}
