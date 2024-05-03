use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Read, Write};
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

    find_matches(reader, &args.pattern, &mut handle)
}

fn find_matches<T>(mut reader: BufReader<T>, pattern: &str, mut writer: impl Write) -> Result<()>
    where T: Read {
    let mut line = String::new();

    while let Ok(len) = reader.read_line(&mut line) {
        if len == 0 {
            break;
        }

        if line.contains(pattern) {
            write!(writer, "{}", line)?;
        }

        line.clear();
    }

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let ok = find_matches(BufReader::new("lorem ipsum\ndolor sit amet".as_bytes()), "lorem", &mut result);
    assert!(ok.is_ok());
    assert_eq!(result, b"lorem ipsum\n");
}
