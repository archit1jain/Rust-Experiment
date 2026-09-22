use log_analyzer::models::LogEntry;
use log_analyzer::stats::Stats;

#[test]
fn test_empty_stats() {
    let stats = Stats::new();
    assert_eq!(stats.total_requests, 0);
    assert!(stats.latencies.is_empty());
}

#[test]
fn test_single_request() {
    let mut stats = Stats::new();
    let entry = LogEntry {
        timestamp: "2026-09-22T10:30:21".to_string(),
        method: "GET".to_string(),
        endpoint: "/api/users".to_string(),
        status: 200,
        response_time_ms: 123,
    };

    stats.add_entry(&entry);

    assert_eq!(stats.total_requests, 1);
    assert_eq!(stats.status_2xx, 1);
    assert_eq!(stats.status_3xx, 0);
    assert_eq!(stats.status_4xx, 0);
    assert_eq!(stats.status_5xx, 0);
    assert_eq!(stats.latencies.len(), 1);
    assert_eq!(stats.latencies[0], 123);

    assert_eq!(*stats.method_counts.get("GET").unwrap(), 1);
    assert_eq!(*stats.endpoint_counts.get("/api/users").unwrap(), 1);
    assert_eq!(*stats.endpoint_latencies.get("/api/users").unwrap(), 123);
}

#[test]
fn test_multiple_requests_and_merge() {
    let mut stats1 = Stats::new();
    let entry1 = LogEntry {
        timestamp: "T1".to_string(),
        method: "GET".to_string(),
        endpoint: "/api/users".to_string(),
        status: 200,
        response_time_ms: 100,
    };
    stats1.add_entry(&entry1);

    let mut stats2 = Stats::new();
    let entry2 = LogEntry {
        timestamp: "T2".to_string(),
        method: "POST".to_string(),
        endpoint: "/api/orders".to_string(),
        status: 500,
        response_time_ms: 200,
    };
    stats2.add_entry(&entry2);

    stats1.merge(stats2);

    assert_eq!(stats1.total_requests, 2);
    assert_eq!(stats1.status_2xx, 1);
    assert_eq!(stats1.status_5xx, 1);

    assert_eq!(stats1.latencies, vec![100, 200]);
    assert_eq!(*stats1.server_errors.get("/api/orders").unwrap(), 1);
}
