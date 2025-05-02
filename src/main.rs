use std::fs::File;
use std::io::Write;
use std::thread;
use std::sync::Arc;
use uuid::Uuid;
use rand::RngCore;
use num_cpus;
use clap::Parser;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::collections::HashSet;
use lazy_static::lazy_static;

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

// Shared set to keep track of created files
lazy_static! {
    static ref CREATED_FILES: Mutex<HashSet<PathBuf>> = Mutex::new(HashSet::new());
}

fn disk_thrash(parent_dir: &PathBuf, buffer: &[u8]) -> std::io::Result<()> {
    // Generate GUID for filename
    let filename = parent_dir.join(format!("{}.tmp", Uuid::new_v4()));

    // Add the filename to the set of created files, for cleanup at CTRL + C
    CREATED_FILES.lock().unwrap().insert(filename.clone());

    // Write to disk
    {
        let mut file = File::create(&filename)?;
        file.write_all(buffer)?;
        file.flush()?;
    }

    // Delete the file
    _ = std::fs::remove_file(&filename);
    
    {
        let filename_lock = CREATED_FILES.lock().unwrap();
        let value = filename_lock.iter().find(|i| *i == &filename);
        match value {
            Some(item) => {
                CREATED_FILES.lock().unwrap().remove(item);
            }
            None => {
                println!("File not found in set");
            }
        }
    }

    Ok(())
}

static STOP_SIGNAL: AtomicBool = AtomicBool::new(false);

fn main() {
    let args = Args::parse();

    // Set up the signal handler for CTRL+C
    ctrlc::set_handler(|| {
        println!("CTRL+C received. Signaling threads to stop...");
        STOP_SIGNAL.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");


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
            while !STOP_SIGNAL.load(Ordering::SeqCst) {
                if let Err(e) = disk_thrash(&parent_dir, &buffer_clone) {
                    eprintln!("Thread {} error: {}", i, e);
                }
            }
            println!("Thread {} stopping", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete (they won't, since they loop infinitely until signal)
    for handle in handles {
        handle.join().unwrap();
    }

    // Clean up created files after threads have stopped
    println!("Threads stopped. Cleaning up temporary files...");
    let files_to_delete = {
        let mut created_files = CREATED_FILES.lock().unwrap();
        created_files.drain().collect::<Vec<_>>()
    };

    for file_path in files_to_delete {
        if let Err(e) = std::fs::remove_file(&file_path) {
            eprintln!("Error deleting file {}: {}", file_path.display(), e);
        } else {
            println!("Deleted {}", file_path.display());
        }
    }
    println!("Cleanup complete. Exiting.");
}