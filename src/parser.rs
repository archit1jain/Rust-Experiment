use crate::error::ParseError;
use crate::models::LogEntry;

/// Parses a single line of log text into a `LogEntry`.
///
/// Takes a borrowed string slice `&str` instead of taking ownership,
/// allowing the caller to retain the original String if they need to.
/// Returns a `Result`: `Ok(LogEntry)` on success, or `Err(ParseError)` on failure.
pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    // We use `split_whitespace()` which creates an Iterator yielding string slices (`&str`).
    // This correctly handles multiple spaces between parts if any exist.
    let mut parts = line.split_whitespace();

    // The `?` operator is not used for `Option`, but we can convert it into a `Result`
    // using `ok_or`. Here, if `next()` returns `None`, it becomes an `Err(ParseError::MissingFields)`.
    // We extract ownership of the string data into a `String` using `to_string()`.
    let timestamp = parts.next().ok_or(ParseError::MissingFields)?.to_string();
    let method = parts.next().ok_or(ParseError::MissingFields)?.to_string();
    let endpoint = parts.next().ok_or(ParseError::MissingFields)?.to_string();

    // Parse the status code.
    // The `parse::<u16>()` method on `&str` returns a `Result<u16, ParseIntError>`.
    // Thanks to the `From` trait implementation in `error.rs`, the `?` operator automatically
    // converts `ParseIntError` into `ParseError::InvalidNumber`.
    let status_str = parts.next().ok_or(ParseError::MissingFields)?;
    let status: u16 = status_str.parse()?;

    // Parse the response time (latency). It expects a format like "123ms".
    let latency_str = parts.next().ok_or(ParseError::MissingFields)?;

    // We check if the string ends with "ms" using the `ends_with` method.
    if !latency_str.ends_with("ms") {
        return Err(ParseError::InvalidLatencyFormat);
    }

    // Slice off the last two characters ("ms"). `len()` gives byte length.
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
