use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum, Debug)]
pub enum Format {
    Console,
    Json,
}

/// The basic configuration derived from command line arguments using `clap`.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The path to the log file we want to process.
    pub file_path: String,

    /// The number of top endpoints to display.
    #[arg(long, default_value_t = 10)]
    pub top: usize,

    /// Output format (console or json)
    #[arg(long, value_enum, default_value_t = Format::Console)]
    pub format: Format,

    /// Only show error statistics
    #[arg(long, default_value_t = false)]
    pub errors_only: bool,

    /// Use asynchronous processing (Tokio)
    #[arg(long, default_value_t = false)]
    pub r#async: bool,
}

impl Config {
    /// Constructs a Config from command line arguments.
    pub fn build() -> Self {
        // clap's Parser trait provides the `parse` method which automatically
        // reads `std::env::args`, parses them according to the struct definition,
        // and handles errors/help messages automatically.
        Config::parse()
    }
}
