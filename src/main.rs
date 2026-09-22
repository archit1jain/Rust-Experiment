use clap::Parser;
use log_analyzer::cli::Config;
use log_analyzer::report::{ConsoleReporter, JsonReporter, Reporter};
use log_analyzer::{process_async, process_sync};

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let mut stats = if config.r#async {
        process_async(&config).await
    } else {
        process_sync(&config)
    };

    match config.format {
        log_analyzer::cli::Format::Console => {
            let reporter = ConsoleReporter;
            reporter.report(&mut stats, config.top, config.errors_only);
        }
        log_analyzer::cli::Format::Json => {
            let reporter = JsonReporter;
            reporter.report(&mut stats, config.top, config.errors_only);
        }
    }
}
