use std::io::prelude::*;
use std::io::BufReader;

use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    let args = Cli::parse();

    let file = std::fs::File::open(&args.path)
        .with_context(|| format!("could not read file: `{}`", &args.path.display()))?;
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
