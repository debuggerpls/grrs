use std::io::prelude::*;
use std::io::BufReader;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let file = std::fs::File::open(args.path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while let Ok(len) = reader.read_line(&mut line) {
        if len == 0 {
            break;
        }

        if line.contains(&args.pattern) {
            print!("{}", line);
        }

        line.clear();
    }

    Ok(())
}
