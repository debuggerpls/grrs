use std::io::prelude::*;
use std::io::{BufReader, Read, Write};

use anyhow::Result;


pub fn find_matches<T>(mut reader: BufReader<T>, pattern: &str, mut writer: impl Write) -> Result<()>
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