// Bring definitions from other parts of the standard library or our own project into scope.
// This allows us to use types and traits like `Parser` and `Config` without writing out their full paths.
use clap::Parser;
use log_analyzer::cli::Config;
use log_analyzer::report::{ConsoleReporter, JsonReporter, Reporter};
use log_analyzer::{process_async, process_sync};

// This is the entry point of our application.
// When a user runs the program, this function starts everything up.
// It reads the user's command line choices, processes the log file accordingly,
// and finally reports the calculated statistics back to the user.

// `#[tokio::main]` is a macro provided by the `tokio` crate.
// It sets up the asynchronous runtime environment that allows our `async fn main()` to run.
// Rust's standard library does not provide an async runtime, so we rely on `tokio` to handle
// running asynchronous tasks and waiting for their completion.
#[tokio::main]
// `async` tells Rust that this function can run asynchronously and can be paused while waiting
// for operations (like reading a file) to complete.
async fn main() {
    // Call the `parse` method provided by `clap::Parser`.
    // This reads the command line arguments provided by the user and creates a `Config` struct.
    // `config` owns this newly created `Config` instance.
    let config = Config::parse();

    // Declare a mutable variable `stats` to hold our computed log statistics.
    // It is `mut` (mutable) because the reporter will need to borrow it mutably later to calculate metrics.
    // We use an `if` expression to evaluate the user's `async` flag and assign the returned `Stats` object to `stats`.
    let mut stats = if config.r#async {
        // If the user requested asynchronous processing, call `process_async`.
        // We pass a shared, immutable borrow (`&config`) because `process_async` only needs
        // to read the configuration, not take ownership or modify it.
        // `.await` pauses this function's execution until `process_async` finishes its work and returns the result.
        process_async(&config).await
    } else {
        // If the user did not request async processing, call the synchronous version.
        // We also pass a shared, immutable borrow (`&config`) here.
        process_sync(&config)
    };

    // `match` is a control flow operator that compares the value of `config.format` against a series of patterns.
    // It ensures we handle every possible enum variant exhaustively.
    match config.format {
        // If the format requested is `Console`:
        log_analyzer::cli::Format::Console => {
            // Create a new instance of our `ConsoleReporter`.
            let reporter = ConsoleReporter;
            // Call the `report` method.
            // We pass a mutable borrow (`&mut stats`) so the reporter can modify `stats`
            // temporarily (e.g., to sort latencies) without taking ownership of it.
            // We also pass copies of `config.top` and `config.errors_only`.
            reporter.report(&mut stats, config.top, config.errors_only);
        }
        // If the format requested is `Json`:
        log_analyzer::cli::Format::Json => {
            // Create a new instance of our `JsonReporter`.
            let reporter = JsonReporter;
            // Call the `report` method just like before, passing a mutable borrow to `stats`.
            reporter.report(&mut stats, config.top, config.errors_only);
        }
    }
}
