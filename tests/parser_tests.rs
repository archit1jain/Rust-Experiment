use log_analyzer::parser::parse_line;

#[test]
fn test_valid_log_line() {
    let line = "2026-09-22T10:30:21 GET /api/users 200 123ms";
    let entry = parse_line(line).expect("Should parse valid line");

    assert_eq!(entry.timestamp, "2026-09-22T10:30:21");
    assert_eq!(entry.method, "GET");
    assert_eq!(entry.endpoint, "/api/users");
    assert_eq!(entry.status, 200);
    assert_eq!(entry.response_time_ms, 123);
}

#[test]
fn test_missing_fields() {
    let line = "2026-09-22T10:30:21 GET /api/users 200";
    let result = parse_line(line);
    assert!(result.is_err(), "Should return error for missing fields");
}

#[test]
fn test_invalid_status() {
    let line = "2026-09-22T10:30:21 GET /api/users INVALID_STATUS 123ms";
    let result = parse_line(line);
    assert!(result.is_err(), "Should return error for invalid status");
}

#[test]
fn test_invalid_latency_format() {
    let line = "2026-09-22T10:30:21 GET /api/users 200 123";
    let result = parse_line(line);
    assert!(result.is_err(), "Should return error for missing 'ms'");
}

#[test]
fn test_empty_line() {
    let line = "";
    let result = parse_line(line);
    assert!(result.is_err());
}
