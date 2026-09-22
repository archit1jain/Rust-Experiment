use std::fmt;
use std::num::ParseIntError;

/// A custom error type to represent various parsing failures.
#[derive(Debug)]
pub enum ParseError {
    /// Indicates the log line doesn't have enough fields (we expect 5).
    MissingFields,
    /// Indicates that a number could not be parsed (e.g., status or latency).
    /// We wrap the standard library's `ParseIntError`.
    InvalidNumber(ParseIntError),
    /// Indicates that the latency format was invalid (missing 'ms').
    InvalidLatencyFormat,
}

// Implement the `Display` trait so our error can be printed nicely to users.
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We use `match` on `self` to provide specific error messages.
        match self {
            ParseError::MissingFields => write!(f, "log line is missing required fields"),
            ParseError::InvalidNumber(err) => write!(f, "invalid number format: {}", err),
            ParseError::InvalidLatencyFormat => {
                write!(f, "latency format is invalid, expected 'ms' suffix")
            }
        }
    }
}

// Implement `std::error::Error` so our custom error integrates nicely with the standard library.
// We can use the default implementation for this simple case.
impl std::error::Error for ParseError {}

// Implement `From<ParseIntError>` to automatically convert `ParseIntError` into our `ParseError`.
// This allows us to use the `?` operator when parsing integers.
impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::InvalidNumber(err)
    }
}
