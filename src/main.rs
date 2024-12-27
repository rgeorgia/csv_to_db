mod cli;

use clap::Parser;
use cli::Args;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Parse the command-line arguments
    let args = Args::parse();
    let directory = args
        .directory
        .unwrap_or_else(|| env::current_dir().unwrap());

    // Get the list of CSV files
    match get_csv_files(&directory) {
        Ok(csv_files) => {
            if csv_files.is_empty() {
                println!("No CSV files found in '{:?}'.", directory.display());
            } else {
                
                println!("Found the following CSV files:");
                for file in csv_files {
                    println!("- {}", file.display());
                }
            }
        }
        Err(err) => {
            eprintln!(
                "Error reading directory '{:?}': {}",
                directory.display(),
                err
            );
        }
    }
}

fn get_csv_files(directory: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    // Check if the directory exists and is a directory
    if directory.is_dir() {
        println!("Searching for CSV files in '{}'...", directory.display());
        // find_csv_files(directory)
        let mut csv_files = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.eq_ignore_ascii_case("csv") {
                        csv_files.push(path);
                    }
                }
            }
        }
        Ok(csv_files)
    } else if directory.exists() {
        println!(
            "The path '{:?}' exists but is not a directory!",
            directory.display()
        );
        Ok(Vec::new())
    } else {
        println!("The directory '{:?}' does not exist.", directory.display());
        Ok(Vec::new())
    }
}
