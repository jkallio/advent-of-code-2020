use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_to_vec(path: &str) -> Result<Vec<String>, std::io::Error> {
    // Open the file from given path
    let file = File::open(path)?;

    // Wrap the file into BufReader to increase performance
    let br = BufReader::new(file);
    let mut vec = Vec::new();

    for line in br.lines() {
        vec.push(line?);
    }
    Ok(vec)
}