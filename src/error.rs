use std::fmt;
use std::num::ParseIntError;

/// A custom error type to represent various parsing failures.
// An enum is used here because an error can only be ONE of these specific failure types at a time.
#[derive(Debug)]
pub enum ParseError {
    /// Indicates the log line doesn't have enough fields (we expect 5).
    MissingFields,
    /// Indicates that a number could not be parsed (e.g., status or latency).
    /// We wrap the standard library's `ParseIntError`.
    // Enums in Rust can hold data! Here, `InvalidNumber` holds the underlying `ParseIntError` from the standard library.
    InvalidNumber(ParseIntError),
    /// Indicates that the latency format was invalid (missing 'ms').
    InvalidLatencyFormat,
}

// A trait is essentially a contract describing what something can do.
// By implementing `fmt::Display`, we are fulfilling a contract that says "this type can be formatted as user-facing text."
// Implement the `Display` trait so our error can be printed nicely to users.
impl fmt::Display for ParseError {
    // `&self` means this method borrows the current `ParseError` instance immutably.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We use `match` on `self` to provide specific error messages.
        // `match` forces us to explicitly handle every possible variant of `ParseError`.
        // If we add a new error type later but forget to handle it here, the compiler will refuse to compile.
        match self {
            ParseError::MissingFields => write!(f, "log line is missing required fields"),
            // Pattern matching allows us to extract the inner data (`err`) bound to `InvalidNumber`.
            ParseError::InvalidNumber(err) => write!(f, "invalid number format: {}", err),
            ParseError::InvalidLatencyFormat => {
                write!(f, "latency format is invalid, expected 'ms' suffix")
            }
        }
    }
}

// The `Error` trait tells the rest of the Rust ecosystem that this type is officially an error type.
// Implement `std::error::Error` so our custom error integrates nicely with the standard library.
// We can use the default implementation for this simple case.
impl std::error::Error for ParseError {}

// Implement `From<ParseIntError>` to automatically convert `ParseIntError` into our `ParseError`.
// This allows us to use the `?` operator when parsing integers.
impl From<ParseIntError> for ParseError {
    // This function takes ownership of `ParseIntError` and returns a new `ParseError`.
    fn from(err: ParseIntError) -> Self {
        ParseError::InvalidNumber(err)
    }
}
