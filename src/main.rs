use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// Command-line arguments for the Directory Verifier
#[derive(Parser, Debug)]
#[command(
    name = "CSV File Finder",
    version = "1.0",
    author = "Ronverbs",
    about = "Lists all CSV files in a directory"
)]

struct Args {
    /// The directory to search for CSV files
    directory: Option<String>,
}

fn default_directory() -> String {
    ".".to_string() // or any default directory path you prefer
}

fn main() {
    // Parse the command-line arguments
    let args = Args::parse();
    let directory = args.directory.unwrap_or_else(default_directory);

    // Check if the directory exists and is a directory
    let path = Path::new(directory.as_str());
    println!("{:?}", path);

    if path.is_dir() {
        println!("Searching for CSV files in '{:?}'...", directory);
        match find_csv_files(path) {
            Ok(csv_files) => {
                if csv_files.is_empty() {
                    println!("No CSV files found in '{:?}'.", directory);
                } else {
                    println!("Found the following CSV files:");
                    for file in csv_files {
                        println!("- {}", file.display());
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading directory '{:?}': {}", directory, err);
            }
        }
    } else if path.exists() {
        println!("The path '{:?}' exists but is not a directory!", directory);
    } else {
        println!("The directory '{:?}' does not exist.", directory);
    }
}

fn find_csv_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut csv_files = Vec::new();
    for entry in fs::read_dir(dir)? {
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
}
