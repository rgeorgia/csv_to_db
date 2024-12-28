mod cli;

use clap::Parser;
use cli::Args;
use std::env;
use std::fs;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::collections::HashMap;

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
    if !directory.exists() {
        println!("The directory '{:?}' does not exist.", directory.display());
        return Ok(Vec::new());
    }

    if !directory.is_dir() {
        println!(
            "The path '{:?}' exists but is not a directory!",
            directory.display()
        );
        return Ok(Vec::new());
    }

    println!("Searching for CSV files in '{}'...", directory.display());
    let csv_files = find_csv_files(directory)?;
    let recent_files = filter_most_recent_files(csv_files);

    Ok(recent_files)
}

fn find_csv_files(directory: &Path) -> Result<HashMap<String, PathBuf>, std::io::Error> {
    let realms = ["COM", "EDM", "EIS"];
    let mut csv_files: HashMap<String, PathBuf> = HashMap::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("csv") {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            for &realm in &realms {
                                if file_name_str.contains(realm) {
                                    update_csv_files(&mut csv_files, realm, &path)?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(csv_files)
}

fn update_csv_files(
    csv_files: &mut HashMap<String, PathBuf>,
    realm: &str,
    path: &Path,
) -> Result<(), std::io::Error> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified()?;

    if let Some(existing_path) = csv_files.get(realm) {
        let existing_metadata = fs::metadata(existing_path)?;
        let existing_modified_time = existing_metadata.modified()?;
        if modified_time > existing_modified_time {
            csv_files.insert(realm.to_string(), path.to_path_buf());
        }
    } else {
        csv_files.insert(realm.to_owned(), path.to_path_buf());
    }

    Ok(())
}

fn filter_most_recent_files(csv_files: HashMap<String, PathBuf>) -> Vec<PathBuf> {
    csv_files.into_values().collect()
}