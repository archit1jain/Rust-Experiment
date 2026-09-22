// `clap` is a popular Rust library (crate) for parsing command line arguments.
use clap::{Parser, ValueEnum};

// An `enum` (enumeration) defines a type that can be one of several different variants.
// Here, `Format` represents the possible ways the user wants to see the output: Console or Json.

// `#[derive(...)]` is a macro that automatically implements certain traits (behaviors) for us.
// - `Clone`: Allows this enum to be easily copied/cloned.
// - `ValueEnum`: A trait from `clap` that lets this enum be used as a command line argument option.
// - `Debug`: Allows us to print the enum for debugging purposes (e.g., using `{:?}`).
#[derive(Clone, ValueEnum, Debug)]
pub enum Format {
    Console,
    Json,
}

/// The basic configuration derived from command line arguments using `clap`.
// A `struct` (structure) groups related data together.
// This `Config` struct will hold all the settings the user provided when starting the application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The path to the log file we want to process.
    pub file_path: String,

    /// The number of top endpoints to display.
    // `#[arg(...)]` is a clap attribute that configures how this field is parsed from the command line.
    #[arg(long, default_value_t = 10)]
    pub top: usize,

    /// Output format (console or json)
    #[arg(long, value_enum, default_value_t = Format::Console)]
    pub format: Format,

    /// Only show error statistics
    #[arg(long, default_value_t = false)]
    pub errors_only: bool,

    /// Use asynchronous processing (Tokio)
    // `r#async` uses the raw identifier syntax `r#`.
    // Because `async` is a reserved keyword in Rust, we must prefix it with `r#` if we want to use it as a variable or field name.
    #[arg(long, default_value_t = false)]
    pub r#async: bool,
}

// `impl` block provides implementations of functions and methods for a specific type (here, `Config`).
impl Config {
    /// Constructs a Config from command line arguments.
    pub fn build() -> Self {
        // clap's Parser trait provides the `parse` method which automatically
        // reads `std::env::args`, parses them according to the struct definition,
        // and handles errors/help messages automatically.
        Config::parse()
    }
}
