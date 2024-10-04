use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use crate::branch::HEAD_FILE;

pub fn current_branch() -> io::Result<String> {
    let head_file = File::open(HEAD_FILE)?;
    let reader = BufReader::new(head_file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("ref:") {
            let branch = line.trim_start_matches("ref: ").to_string();
            return Ok(branch);
        }
    }

    Err(io::Error::new(io::ErrorKind::Other, "No se pudo determinar la rama actual"))
}