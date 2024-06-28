use clap::Parser;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

/// CLI tool to hash a file
#[derive(Parser)]
struct Cli {
    /// The command we are running
    command: String,
    /// The path to the file we are hashing
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    match args.command.as_str() {
        "baseline" => {
            match calculate_sha256(&args.path) {
                Ok(hash) => println!("SHA-256 hash: {}", hash),
                Err(e) => eprintln!("Error hashing file: {}", e),
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args.command);
            print_usage();
        }
    }
}

fn calculate_sha256(file_path: &PathBuf) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn print_usage() {
    println!("Usage:");
    println!("   <command> <file>");
    println!("Commands:");
    println!("   baseline <file> - Calculate the SHA-256 baseline hash of the file");
}

