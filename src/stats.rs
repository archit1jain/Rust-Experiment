use std::collections::HashMap;

use crate::models::LogEntry;

// This module acts as the "statistics engine" of the application.
// It receives parsed log entries one by one and updates its internal counters and lists.

/// Holds the computed statistics for the log entries.
// The `Default` derive macro automatically implements a `default()` method that
// initializes all fields to their default values (e.g., `0` for integers, empty for `Vec` and `HashMap`).
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
    // `Self` refers to the type this `impl` block is for (which is `Stats`).
    pub fn new() -> Self {
        Stats::default()
    }

    /// Aggregates a single log entry into our statistics.
    // `&mut self` is a mutable borrow. It means this method requires temporary, exclusive access
    // to the `Stats` object so it can modify its internal fields (like incrementing `total_requests`).
    // `&LogEntry` is an immutable borrow. We only need to read the entry, not change it or take ownership.
    pub fn add_entry(&mut self, entry: &LogEntry) {
        self.total_requests += 1;
        self.latencies.push(entry.response_time_ms);

        // `match` is like a powerful `switch` statement.
        // Here we match the integer `status` against ranges (e.g., `200..=299`).
        match entry.status {
            200..=299 => self.status_2xx += 1,
            300..=399 => self.status_3xx += 1,
            400..=499 => self.status_4xx += 1,
            500..=599 => {
                self.status_5xx += 1;
                // `.entry()` checks if the key exists in the HashMap.
                // `.clone()` creates an owned String because the HashMap needs to own its keys.
                // `.or_insert(0)` inserts `0` if the key didn't exist, and returns a mutable reference to the value.
                // The `*` (dereference operator) follows that mutable reference so we can add `1` directly to the actual stored value.
                *self
                    .server_errors
                    .entry(entry.endpoint.clone())
                    .or_insert(0) += 1;
            }
            // The `_` is a catch-all pattern. If the status doesn't match any of the above, do nothing (`{}`).
            _ => {}
        }

        // Just like above, we increment the counts for this specific endpoint.
        *self
            .endpoint_counts
            .entry(entry.endpoint.clone())
            .or_insert(0) += 1;

        // We add the latency to the total latency for this endpoint.
        *self
            .endpoint_latencies
            .entry(entry.endpoint.clone())
            .or_insert(0) += entry.response_time_ms;

        // We increment the count for this specific HTTP method (GET, POST, etc).
        *self.method_counts.entry(entry.method.clone()).or_insert(0) += 1;
    }

    /// Merges another Stats object into this one.
    /// This is very useful for concurrent processing where each thread returns its own Stats.
    // Notice `other: Stats` does not have a `&`. This means this method takes *ownership* of the `other` object.
    // The `other` object will be destroyed when this method finishes.
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
            // `for (k, v) in src` consumes `src` (the HashMap) and iterates over its keys and values.
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
