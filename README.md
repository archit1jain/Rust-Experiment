# Log Analyzer

A high-performance CLI tool for parsing and analyzing HTTP access logs, written in Rust.

## Project Overview

`log-analyzer` reads log files containing HTTP request data, parses each line, aggregates statistics (such as total requests, latencies, HTTP status codes, endpoints, etc.), and reports the findings either to the console or as JSON. The tool supports multiple processing modes: synchronous multi-threaded (using standard library threads and channels) and asynchronous (using the Tokio runtime).

## Architecture

The project is structured modularly:
- **`src/main.rs` & `src/lib.rs`**: Entry point and core processing paths (sync & async).
- **`src/cli.rs`**: Command-line argument parsing using `clap`.
- **`src/models.rs`**: Data structures (e.g., `LogEntry`).
- **`src/parser.rs`**: Log parsing logic with proper error handling.
- **`src/stats.rs`**: Aggregation of metrics (latencies, counts, etc.).
- **`src/report.rs`**: Output formatting via the `Reporter` trait.
- **`src/error.rs`**: Custom error types (`ParseError`).

## Input Format

The analyzer expects log lines in the following format:
```text
timestamp method endpoint status response_time
```

Example:
```text
2026-09-22T10:30:21 GET /api/users 200 123ms
```

## Installation

Ensure you have Rust installed, then clone the repo and build the project:

```bash
cargo build --release
```

## Usage

Run the tool by providing a path to a log file:

```bash
cargo run --release -- access.log
```

## CLI Options

- `--top <NUM>`: Number of top endpoints to display (default: 10)
- `--format <console|json>`: Output format (default: console)
- `--errors-only`: Only show error statistics
- `--async`: Use asynchronous processing (Tokio) instead of synchronous threads

Example commands:
```bash
cargo run -- access.log --top 20
cargo run -- access.log --format json
cargo run -- access.log --errors-only
cargo run -- access.log --async
```

## Concurrency Model

The application supports two concurrency models:
1. **Synchronous Multi-threaded**: Uses `std::thread` to spawn worker threads. The main thread reads the file and distributes lines to workers using `mpsc` (Multi-Producer, Single-Consumer) channels.
2. **Asynchronous (Tokio)**: Uses Tokio tasks (`tokio::spawn`) and asynchronous channels (`tokio::sync::mpsc`) to process log lines concurrently without blocking OS threads.

## Performance

A benchmarking script (`generate_logs.sh`) was used to create a 1,000,000 line synthetic log file. The multi-threaded implementation processes the file in a fraction of a second, efficiently utilizing all available CPU cores.

## Testing

The project includes comprehensive unit tests for parsing and statistics calculations. Run the test suite using:

```bash
cargo test
```

## Dependencies

- **`clap`**: For parsing complex CLI arguments elegantly.
- **`serde` & `serde_json`**: For serializing statistics into JSON format.
- **`tokio`**: For providing the asynchronous runtime and async concurrency primitives.

## Rust Concepts Demonstrated

This project showcases several core Rust concepts:
- **Structs and Enums**: Modeling data (`LogEntry`, `Stats`, `ParseError`).
- **Traits and Impls**: Defining shared behavior (`Reporter`, `std::fmt::Display`).
- **Error Handling**: Custom error types, `Result`, `Option`, and the `?` operator.
- **Pattern Matching**: Using `match` and `if let` for control flow.
- **Ownership and Borrowing**: Passing references (`&str`, `&LogEntry`) to avoid unnecessary allocations.
- **Collections**: Leveraging `Vec` for latencies and `HashMap` for counting occurrences.
- **Iterators and Closures**: For processing collections efficiently.
- **Concurrency**: Using `std::thread`, `mpsc` channels, `tokio::spawn`, and async/await.
- **Modules**: Organizing code cleanly into separate files.
