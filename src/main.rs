use std::fs::File;
use std::io::Write;
use std::thread;
use uuid::Uuid;
use rand::Rng;
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
}

fn disk_thrash(parent_dir: &PathBuf) -> std::io::Result<()> {
    // Provision 500 MB byte array
    let size = 500 * 1024 * 1024; // 500 MB in bytes
    let mut buffer = vec![0u8; size];

    // Fill with random data
    rand::rng().fill(&mut buffer[..]);

    // Generate GUID for filename
    let filename = parent_dir.join(format!("{}.tmp", Uuid::new_v4()));

    // Write to disk
    {
        let mut file = File::create(&filename)?;
        file.write_all(&buffer)?;
        file.flush()?;
    }

    // Delete the file
    std::fs::remove_file(&filename)?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    // Get number of CPUs
    let num_threads = num_cpus::get();
    println!("Spawning {} threads", num_threads);

    // Create a vector to hold thread handles
    let mut handles = vec![];

    // Spawn threads
    for i in 0..num_threads {
        let parent_dir = args.parent_dir.clone();
        let handle = thread::spawn(move || {
            println!("Thread {} started", i);
            loop {
                if let Err(e) = disk_thrash(&parent_dir) {
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