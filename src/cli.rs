use clap::Parser;
use std::env;
use std::path::PathBuf;

/// Command-line arguments for the Directory Verifier
#[derive(Parser, Debug)]
#[command(
    name = "CSV File Finder",
    version = "1.0",
    author = "Ronverbs",
    about = "Lists all CSV files in a directory"
)]
pub struct Args {
    /// The directory to search for CSV files
    pub directory: Option<PathBuf>,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            directory: Some(env::current_dir().unwrap()),
        }
    }
}
