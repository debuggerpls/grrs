use std::io::{self, BufReader, BufWriter};
use std::string::String;

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
    let reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    grrs::find_matches(reader, &args.pattern, &mut handle)
}
