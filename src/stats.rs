use std::collections::HashMap;

use crate::models::LogEntry;

/// Holds the computed statistics for the log entries.
#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub total_requests: u64,
    pub status_2xx: u64,
    pub status_3xx: u64,
    pub status_4xx: u64,
    pub status_5xx: u64,

    pub latencies: Vec<u64>,
    pub endpoint_counts: HashMap<String, u64>,
    pub endpoint_latencies: HashMap<String, u64>,
    pub method_counts: HashMap<String, u64>,
    pub server_errors: HashMap<String, u64>,
}

impl Stats {
    /// Creates a new, empty Stats object.
    pub fn new() -> Self {
        Stats::default()
    }

    /// Aggregates a single log entry into our statistics.
    pub fn add_entry(&mut self, entry: &LogEntry) {
        self.total_requests += 1;
        self.latencies.push(entry.response_time_ms);

        match entry.status {
            200..=299 => self.status_2xx += 1,
            300..=399 => self.status_3xx += 1,
            400..=499 => self.status_4xx += 1,
            500..=599 => {
                self.status_5xx += 1;
                *self
                    .server_errors
                    .entry(entry.endpoint.clone())
                    .or_insert(0) += 1;
            }
            _ => {}
        }

        *self
            .endpoint_counts
            .entry(entry.endpoint.clone())
            .or_insert(0) += 1;
        *self
            .endpoint_latencies
            .entry(entry.endpoint.clone())
            .or_insert(0) += entry.response_time_ms;
        *self.method_counts.entry(entry.method.clone()).or_insert(0) += 1;
    }

    /// Merges another Stats object into this one.
    /// This is very useful for concurrent processing where each thread returns its own Stats.
    pub fn merge(&mut self, other: Stats) {
        self.total_requests += other.total_requests;
        self.status_2xx += other.status_2xx;
        self.status_3xx += other.status_3xx;
        self.status_4xx += other.status_4xx;
        self.status_5xx += other.status_5xx;

        // Extend our latencies vector with the other's elements.
        self.latencies.extend(other.latencies);

        // Helper to merge HashMaps by summing their values.
        fn merge_maps(dest: &mut HashMap<String, u64>, src: HashMap<String, u64>) {
            for (k, v) in src {
                *dest.entry(k).or_insert(0) += v;
            }
        }

        merge_maps(&mut self.endpoint_counts, other.endpoint_counts);
        merge_maps(&mut self.endpoint_latencies, other.endpoint_latencies);
        merge_maps(&mut self.method_counts, other.method_counts);
        merge_maps(&mut self.server_errors, other.server_errors);
    }
}
