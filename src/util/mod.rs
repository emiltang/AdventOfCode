use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Result;

pub fn read(file_name: &str) -> Result<Vec<String>> {
    let file = File::open(file_name)?;
    let buffer = BufReader::new(file);
    return buffer.lines().collect();
}
