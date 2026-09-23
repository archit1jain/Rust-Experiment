use crate::stats::Stats;
use serde::Serialize;
use serde_json;

// This module is responsible for formatting and displaying the calculated statistics.
// It converts the raw numbers into a human-readable format or a computer-readable format (JSON),
// depending on what the user requested.

// A trait is essentially a contract describing what something can do.
// By creating a `Reporter` trait, we guarantee that any struct implementing it will have a `report` method.
// This allows us to easily swap out how we print statistics without changing the core analysis code.
/// A trait defining how a reporter should output statistics.
/// This allows us to easily swap between different output formats.
pub trait Reporter {
    // `&self` means the method borrows the reporter immutably.
    // `&mut stats` means it borrows the `Stats` object mutably (so it can modify it, e.g., by sorting arrays inside it).
    fn report(&self, stats: &mut Stats, top_n: usize, errors_only: bool);
}

/// A reporter that prints a human-readable table to the console.
pub struct ConsoleReporter;

// Here we implement the `Reporter` trait contract for our `ConsoleReporter`.
impl Reporter for ConsoleReporter {
    fn report(&self, stats: &mut Stats, top_n: usize, errors_only: bool) {
        if !errors_only {
            // Calculate latencies (we sort the vector to find percentiles)
            let (_min, _max, avg, p50, p95, p99) = calculate_latency_metrics(stats);

            println!("╔══════════════════════════════════════╗");
            println!("║            LOG ANALYZER              ║");
            println!("╠══════════════════════════════════════╣");
            println!("║ Total Requests   : {:<18}║", stats.total_requests);
            println!("║ Successful (2xx) : {:<18}║", stats.status_2xx);
            println!("║ Redirects (3xx)  : {:<18}║", stats.status_3xx);
            println!("║ Client Errors    : {:<18}║", stats.status_4xx);
            println!("║ Server Errors    : {:<18}║", stats.status_5xx);
            println!("║ Average Latency  : {:<16}ms║", avg);
            println!("║ P50 Latency      : {:<16}ms║", p50);
            println!("║ P95 Latency      : {:<16}ms║", p95);
            println!("║ P99 Latency      : {:<16}ms║", p99);
            println!("╚══════════════════════════════════════╝\n");

            println!("HTTP Methods:");
            println!("────────────────────────");
            // `iter()` creates an iterator over the key-value pairs in the HashMap.
            // `collect()` gathers all these pairs into a new `Vec`. We do this because HashMaps have no order, but we want to sort them.
            let mut methods: Vec<_> = stats.method_counts.iter().collect();
            // `sort_by` takes a closure (a small, anonymous function).
            // `|a, b|` defines the parameters the closure receives. It compares the counts (the second element, `.1`) to sort in descending order.
            methods.sort_by(|a, b| b.1.cmp(a.1));
            for (method, count) in methods {
                println!("{:<8} {}", method, count);
            }
            println!();

            println!("Top {} Endpoints:", top_n);
            println!("────────────────────────");
            let mut endpoints: Vec<_> = stats.endpoint_counts.iter().collect();
            endpoints.sort_by(|a, b| b.1.cmp(a.1));
            // `into_iter()` consumes the `endpoints` Vec, giving us ownership of its elements.
            // `take(top_n)` limits the iterator to yield only the first `top_n` items.
            for (endpoint, count) in endpoints.into_iter().take(top_n) {
                // `get()` returns an `Option`. If the endpoint isn't found (which shouldn't happen here), `unwrap_or(&0)` provides a safe default of 0.
                let avg_latency = stats.endpoint_latencies.get(endpoint).unwrap_or(&0) / count;
                println!("{:<20} {} (avg: {}ms)", endpoint, count, avg_latency);
            }
            println!();
        }

        if errors_only || !stats.server_errors.is_empty() {
            println!("5xx Errors by Endpoint");
            println!("────────────────────────");
            let mut errors: Vec<_> = stats.server_errors.iter().collect();
            errors.sort_by(|a, b| b.1.cmp(a.1));
            for (endpoint, count) in errors {
                println!("{:<20} {}", endpoint, count);
            }
        }
    }
}

/// A struct used for JSON serialization.
/// We map the fields from our internal `Stats` structure to this one.
// `#[derive(Serialize)]` tells the `serde` crate to automatically write code that can turn this struct into JSON format.
// The `'a` is a lifetime annotation. It tells the Rust compiler: "The HashMaps this struct borrows must live at least as long as this JsonReport object itself."
#[derive(Serialize)]
struct JsonReport<'a> {
    total_requests: u64,
    status_2xx: u64,
    status_3xx: u64,
    status_4xx: u64,
    status_5xx: u64,
    min_latency_ms: u64,
    max_latency_ms: u64,
    avg_latency_ms: u64,
    p50_latency_ms: u64,
    p95_latency_ms: u64,
    p99_latency_ms: u64,
    method_counts: &'a std::collections::HashMap<String, u64>,
    endpoint_counts: &'a std::collections::HashMap<String, u64>,
    server_errors: &'a std::collections::HashMap<String, u64>,
}

/// A reporter that outputs statistics as a JSON string.
pub struct JsonReporter;

// Implement the `Reporter` trait for JSON output.
impl Reporter for JsonReporter {
    fn report(&self, stats: &mut Stats, _top_n: usize, _errors_only: bool) {
        // Calculate the latencies first.
        let (min, max, avg, p50, p95, p99) = calculate_latency_metrics(stats);

        // Build the `JsonReport` struct, borrowing the HashMaps directly from the `stats` object to avoid unnecessary copying.
        let report = JsonReport {
            total_requests: stats.total_requests,
            status_2xx: stats.status_2xx,
            status_3xx: stats.status_3xx,
            status_4xx: stats.status_4xx,
            status_5xx: stats.status_5xx,
            min_latency_ms: min,
            max_latency_ms: max,
            avg_latency_ms: avg,
            p50_latency_ms: p50,
            p95_latency_ms: p95,
            p99_latency_ms: p99,
            method_counts: &stats.method_counts,
            endpoint_counts: &stats.endpoint_counts,
            server_errors: &stats.server_errors,
        };

        // Serialize the report to a formatted JSON string and print it.
        // `to_string_pretty` returns a `Result`. `if let Ok(json)` is a concise way to say: "If this succeeded and returned `Ok`, assign the value to `json` and run the block."
        if let Ok(json) = serde_json::to_string_pretty(&report) {
            println!("{}", json);
        }
    }
}

/// Helper function to calculate latencies from the Stats struct.
// This function takes a mutable borrow of `Stats` because it needs to sort the internal `latencies` vector.
fn calculate_latency_metrics(stats: &mut Stats) -> (u64, u64, u64, u64, u64, u64) {
    if stats.latencies.is_empty() {
        return (0, 0, 0, 0, 0, 0);
    }

    // Sort latencies to find min, max and percentiles.
    // `sort_unstable` modifies the vector in place. This is why we needed a mutable borrow (`&mut stats`).
    stats.latencies.sort_unstable();

    let len = stats.latencies.len();
    let min = stats.latencies[0];
    let max = stats.latencies[len - 1];

    // Compute the sum using an iterator.
    // `.iter()` produces the values, and `.sum()` consumes them to add them all up.
    let sum: u64 = stats.latencies.iter().sum();
    let avg = sum / len as u64;

    // Calculate percentiles by accessing the vector at specific calculated indices.
    let p50 = stats.latencies[(len as f64 * 0.50) as usize];
    let p95 = stats.latencies[(len as f64 * 0.95) as usize];
    let p99 = stats.latencies[(len as f64 * 0.99) as usize];

    // Return the results as a tuple.
    (min, max, avg, p50, p95, p99)
}
