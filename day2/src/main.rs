use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// Split file into vector of strings
fn split_file(path: &str) -> Result<Vec<String>, std::io::Error> {    
    // Open the file and wrap it into generic buffered reader
    let file = File::open(path)?; // '?' Early exit on error
    let br = BufReader::new(file);

    let mut vec = Vec::new();
    for line in br.lines() {
        let line = line?; // '?' Early exit as IO operations may return errors
        vec.push(line);
    }
    return Ok(vec);
}

// Main function
fn main() {
    let filename = "input.txt";
    let result = split_file(filename);
    match result {
        Ok(v) => {
            for x in v {
                println!("{}", x)
            }
        },
        Err(e) => {
            println!("*** Error: {}", e);
        }
    }
}
