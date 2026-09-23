// A LogEntry represents one request recorded by our server.
// Think of it like one row in a spreadsheet: it tells us when the request
// happened, which API was called, what HTTP method was used, whether the
// request succeeded, and how long the server took to respond.
//
// `#[derive(...)]` tells the Rust compiler to automatically write basic implementations for:
// - `Debug`: Allowing us to print it via `{:?}`.
// - `PartialEq`: Allowing us to compare two `LogEntry` instances with `==`.
// - `Clone`: Allowing us to create a deep copy of a `LogEntry`.
#[derive(Debug, PartialEq, Clone)]
pub struct LogEntry {
    /// We store the timestamp as a String since we initially read it as text.
    // `String` is an owned, growable piece of text stored on the heap.
    // We use `String` here instead of a borrowed string slice (`&str`) because a `LogEntry`
    // needs to own its data so it can be safely passed around between threads and kept alive
    // independently of the original log file buffer.
    pub timestamp: String,

    /// The HTTP method, like GET, POST, etc.
    pub method: String,

    /// The requested API endpoint, e.g., /api/users.
    pub endpoint: String,

    /// HTTP status codes fit nicely inside an unsigned 16-bit integer.
    pub status: u16,

    /// Response latency in milliseconds.
    pub response_time_ms: u64,
}
