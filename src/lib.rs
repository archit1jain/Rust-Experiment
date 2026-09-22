pub mod cli;
pub mod error;
pub mod models;
pub mod parser;
pub mod report;
pub mod stats;

use cli::Config;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;
use std::sync::mpsc;
use std::thread;

use crate::parser::parse_line;
use crate::stats::Stats;

/// The synchronous, multi-threaded processing logic.
pub fn process_sync(config: &Config) -> Stats {
    let num_workers = 4;
    let mut senders = Vec::new();
    let mut handles = Vec::new();

    for _ in 0..num_workers {
        let (tx, rx) = mpsc::channel::<String>();
        senders.push(tx);

        let handle = thread::spawn(move || {
            let mut local_stats = Stats::new();
            for line in rx {
                if let Ok(entry) = parse_line(&line) {
                    local_stats.add_entry(&entry);
                }
            }
            local_stats
        });
        handles.push(handle);
    }

    match File::open(&config.file_path) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                if let Ok(l) = line {
                    let _ = senders[index % num_workers].send(l);
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening file '{}': {}", config.file_path, e);
            process::exit(1);
        }
    }

    drop(senders);

    let mut stats = Stats::new();
    for handle in handles {
        if let Ok(local_stats) = handle.join() {
            stats.merge(local_stats);
        }
    }

    stats
}

/// The asynchronous processing logic using Tokio.
pub async fn process_async(config: &Config) -> Stats {
    // We use tokio's async file I/O and channels.
    use tokio::fs::File as AsyncFile;
    use tokio::io::{AsyncBufReadExt, BufReader as AsyncBufReader};
    use tokio::sync::mpsc as async_mpsc;
    use tokio::task;

    let num_workers = 4;
    let mut senders = Vec::new();
    let mut handles = Vec::new();

    // Spawn the async worker tasks.
    for _ in 0..num_workers {
        // async_mpsc channels require a capacity limit (bounded).
        let (tx, mut rx) = async_mpsc::channel::<String>(1000);
        senders.push(tx);

        // tokio::spawn schedules a task on the async runtime.
        let handle = task::spawn(async move {
            let mut local_stats = Stats::new();
            // `.recv().await` asynchronously waits for the next message.
            while let Some(line) = rx.recv().await {
                if let Ok(entry) = parse_line(&line) {
                    local_stats.add_entry(&entry);
                }
            }
            local_stats
        });

        handles.push(handle);
    }

    // Await opening the file asynchronously.
    match AsyncFile::open(&config.file_path).await {
        Ok(file) => {
            let reader = AsyncBufReader::new(file);
            let mut lines = reader.lines();
            let mut index = 0;

            // Asynchronously read lines.
            while let Ok(Some(line)) = lines.next_line().await {
                // Send the line asynchronously.
                let _ = senders[index % num_workers].send(line).await;
                index += 1;
            }
        }
        Err(e) => {
            eprintln!("Error opening file '{}': {}", config.file_path, e);
            process::exit(1);
        }
    }

    // Drop senders to signal workers to stop.
    drop(senders);

    let mut stats = Stats::new();
    // Await all tasks to finish and collect their results.
    for handle in handles {
        if let Ok(local_stats) = handle.await {
            stats.merge(local_stats);
        }
    }

    stats
}
