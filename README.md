# Log Analyzer

A high-performance CLI tool for parsing and analyzing HTTP access logs, written in Rust.

## Project Overview

`log-analyzer` reads log files containing HTTP request data, parses each line, aggregates statistics (such as total requests, latencies, HTTP status codes, endpoints, etc.), and reports the findings either to the console or as JSON. The tool supports multiple processing modes: synchronous multi-threaded (using standard library threads and channels) and asynchronous (using the Tokio runtime).

## How This Project Works

A user gives the application a log file.

        ↓

The application reads the file line by line.

        ↓

Each line is converted into structured information (a `LogEntry`).
*(Handled by `parser.rs`)*

        ↓

The structured information is passed to the statistics engine.
*(Orchestrated by `lib.rs` using worker threads or async tasks)*

        ↓

The statistics engine calculates request counts, error rates, and response-time information.
*(Handled by `stats.rs`)*

        ↓

The reporting layer converts those results into human-readable tables or JSON data.
*(Handled by `report.rs`)*

        ↓

The user sees the final report in their terminal.
*(Triggered by `main.rs`)*

## Architecture

The project is structured modularly:
- **`src/main.rs`**: The entry point of the application. It reads user input and triggers the analysis.
- **`src/lib.rs`**: The core execution engine. It handles reading the file and passing lines to worker threads/tasks.
- **`src/cli.rs`**: Defines the command-line arguments the user can provide.
- **`src/models.rs`**: Defines the fundamental data shapes, like the `LogEntry` structure.
- **`src/parser.rs`**: Responsible for understanding the raw log text and converting it into structured data.
- **`src/stats.rs`**: The "calculator" that adds up counts, tracks latencies, and categorizes errors.
- **`src/report.rs`**: The output layer that displays the final calculated data.
- **`src/error.rs`**: Defines the specific ways the parsing process can fail.

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

## Rust Concepts Used

This project showcases several core Rust concepts, explained directly in the code:

- **Ownership:** Explicitly moving data versus passing it (`src/stats.rs` during merging).
- **Borrowing:** Passing references to avoid copying (`&config` in `src/main.rs`).
- **Mutable Borrowing:** Modifying data temporarily (`&mut stats` in `src/report.rs`).
- **String vs &str:** Owned text (`String` in `LogEntry`) vs borrowed views (`&str` in `src/parser.rs`).
- **Structs:** Grouping data together (`Config` in `src/cli.rs`, `LogEntry` in `src/models.rs`).
- **Enums:** Defining a type with multiple variants (`Format` in `src/cli.rs`, `ParseError` in `src/error.rs`).
- **Traits:** Defining shared behavior contracts (`Reporter` in `src/report.rs`, `Display` in `src/error.rs`).
- **Generics:** Used broadly via standard library types like `Vec<T>` and `Result<T, E>`.
- **Option:** Handling missing values gracefully (`src/parser.rs` with iterators).
- **Result:** Handling successes and failures explicitly (`src/parser.rs`).
- **The `?` Operator:** Propagating errors automatically (`src/parser.rs`).
- **Pattern Matching:** Deconstructing enums or ranges (`match` on status codes in `src/stats.rs`).
- **Iterators:** Processing data efficiently step-by-step (`lines()` in `src/lib.rs`, `split_whitespace()` in `src/parser.rs`).
- **Closures:** Small anonymous functions (`thread::spawn` in `src/lib.rs`, sorting in `src/report.rs`).
- **Collections:** Storing lists and mappings (`Vec` and `HashMap` in `src/stats.rs`).
- **Error Handling:** Creating custom error domains (`src/error.rs`).
- **Modules:** Organizing namespaces (`pub mod` declarations in `src/lib.rs`).
- **Concurrency:** Utilizing OS threads and `mpsc` channels (`process_sync` in `src/lib.rs`).
- **Async Rust:** Utilizing Tokio tasks and `await` (`process_async` in `src/lib.rs`).
- **Lifetimes:** Expressing how long borrowed data lives (`'a` in `JsonReport` within `src/report.rs`).
