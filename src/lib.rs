// Declare our sub-modules. In Rust, a module represents a namespace.
// These declarations tell the compiler to look for files with these names
// (e.g., `cli.rs`, `error.rs`) and compile them as part of this library.
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
// This function orchestrates reading the file and parsing it concurrently
// using operating system threads.
pub fn process_sync(config: &Config) -> Stats {
    let num_workers = 4;
    // `senders` holds the transmitting ends of our channels.
    let mut senders = Vec::new();
    // `handles` holds references to the running threads, allowing us to wait for them.
    let mut handles = Vec::new();

    // Create our worker threads.
    for _ in 0..num_workers {
        // `mpsc::channel` creates a Multi-Producer, Single-Consumer channel.
        // It gives us a transmitter (`tx`) and a receiver (`rx`).
        // We will use this to send lines of text from the main thread to the worker thread.
        let (tx, rx) = mpsc::channel::<String>();
        senders.push(tx);

        // `thread::spawn` creates a new OS thread.
        // The `move` keyword forces the closure to take ownership of the variables it uses from the surrounding scope.
        // In this case, it takes ownership of `rx` (the receiving end of the channel) so it can read from it safely in the new thread.
        let handle = thread::spawn(move || {
            let mut local_stats = Stats::new();
            // This loop pulls messages from the channel until the channel is closed.
            for line in rx {
                // If `parse_line` succeeds, we get the structured `entry` and add it to our local stats.
                if let Ok(entry) = parse_line(&line) {
                    local_stats.add_entry(&entry);
                }
            }
            // Return the accumulated statistics for this specific thread.
            local_stats
        });
        handles.push(handle);
    }

    // Attempt to open the log file specified in the configuration.
    match File::open(&config.file_path) {
        // `Ok(file)` means the file was opened successfully.
        Ok(file) => {
            // `BufReader` adds buffering, making line-by-line reading much faster.
            let reader = io::BufReader::new(file);
            // `.lines()` creates an Iterator over the lines.
            // `.enumerate()` wraps the iterator to provide the current index alongside each line.
            for (index, line) in reader.lines().enumerate() {
                // If reading the line was successful...
                if let Ok(l) = line {
                    // Send the line to one of our worker threads using a round-robin approach.
                    let _ = senders[index % num_workers].send(l);
                }
            }
        }
        // `Err(e)` means an error occurred (e.g., file not found).
        Err(e) => {
            eprintln!("Error opening file '{}': {}", config.file_path, e);
            // Exit the entire application with an error code.
            process::exit(1);
        }
    }

    // `drop(senders)` intentionally destroys our copies of the channel transmitters.
    // When all transmitters for a channel are dropped, the channel is considered "closed".
    // This tells the worker threads (which are waiting on `rx`) that no more messages will ever arrive,
    // allowing their `for line in rx` loops to terminate gracefully.
    drop(senders);

    // Create a final Stats object to combine all the thread-local results.
    let mut stats = Stats::new();

    // Wait for all worker threads to finish.
    for handle in handles {
        // `.join()` blocks the main thread until the worker thread completes.
        // If the thread finishes normally, it returns the `local_stats` we computed.
        if let Ok(local_stats) = handle.join() {
            // Combine the thread's results into our global results.
            stats.merge(local_stats);
        }
    }

    stats
}

/// The asynchronous processing logic using Tokio.
// This function orchestrates processing using the Tokio async runtime instead of OS threads.
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
        // This prevents memory exhaustion if the producer reads lines faster than workers can process them.
        let (tx, mut rx) = async_mpsc::channel::<String>(1000);
        senders.push(tx);

        // `tokio::spawn` schedules an async task on the Tokio runtime.
        // Similar to `thread::spawn`, `move` transfers ownership of `rx` to the async closure.
        let handle = task::spawn(async move {
            let mut local_stats = Stats::new();
            // `.recv().await` asynchronously waits for the next message from the channel.
            // When the channel is closed (senders are dropped), this returns `None` and the loop ends.
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
    // This does not block the underlying OS thread, allowing the Tokio runtime to perform other tasks while waiting for the disk.
    match AsyncFile::open(&config.file_path).await {
        Ok(file) => {
            let reader = AsyncBufReader::new(file);
            let mut lines = reader.lines();
            let mut index = 0;

            // Asynchronously read lines one by one.
            while let Ok(Some(line)) = lines.next_line().await {
                // Send the line asynchronously to the appropriate worker.
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
    // Just like in the synchronous version, this closes the channels.
    drop(senders);

    let mut stats = Stats::new();
    // Await all async tasks to finish and collect their results.
    for handle in handles {
        // `.await` pauses execution here until the task finishes.
        if let Ok(local_stats) = handle.await {
            stats.merge(local_stats);
        }
    }

    stats
}
