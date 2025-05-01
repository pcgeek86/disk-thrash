use std::fs::File;
use std::io::Write;
use std::thread;
use std::sync::Arc;
use uuid::Uuid;
use rand::RngCore;
use num_cpus;
use clap::Parser;
use std::path::PathBuf;

/// Disk thrashing tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Parent directory for random files
    #[arg(short, long, default_value = ".")]
    parent_dir: PathBuf,
    #[arg(short, long, default_value = "100", help = "Randomized buffer size in MB")]
    buffer_size: usize,

}

fn disk_thrash(parent_dir: &PathBuf, buffer: &[u8]) -> std::io::Result<()> {
    // Generate GUID for filename
    let filename = parent_dir.join(format!("{}.tmp", Uuid::new_v4()));

    

    // Write to disk
    {
        let mut file = File::create(&filename)?;
        file.write_all(buffer)?;
        file.flush()?;
    }

    // Delete the file
    std::fs::remove_file(&filename)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    // Provision 100 MB byte array (only once)
    let size = args.buffer_size * 1024 * 1024; // 100 MB in bytes
    let mut buffer = vec![0u8; size];

    // Fill with random data (only once)
    rand::rng().fill_bytes(&mut buffer);

    // Wrap the buffer in an Arc for thread-safe sharing
    let shared_buffer = Arc::new(buffer);

    // Get number of CPUs
    let num_threads = num_cpus::get();
    println!("Spawning {} threads", num_threads);

    // Create a vector to hold thread handles
    let mut handles = vec![];

    // Spawn threads
    for i in 0..num_threads {
        let parent_dir = args.parent_dir.clone();
        let buffer_clone = Arc::clone(&shared_buffer);
        let handle = thread::spawn(move || {
            println!("Thread {} started", i);
            loop {
                if let Err(e) = disk_thrash(&parent_dir, &buffer_clone) {
                    eprintln!("Thread {} error: {}", i, e);
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete (they won't, since they loop infinitely)
    for handle in handles {
        handle.join().unwrap();
    }
}