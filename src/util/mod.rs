use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Result;



pub fn read(file_name: &str) -> Vec<String>{
    let file = File::open(file_name).expect("Error reading file");
    let buffer = BufReader::new(file);
    return buffer.lines().map(|s| s.unwrap()).collect();
}
