/// Represents a single parsed log entry.
#[derive(Debug, PartialEq, Clone)]
pub struct LogEntry {
    /// We store the timestamp as a String since we initially read it as text.
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
