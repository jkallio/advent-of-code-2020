use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

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

// Returns parsed min and max values from input string (e.g. "1-7")
fn parse_min_max(range: &str) -> Result<(i32, i32), std::io::Error> {
    let mut it = range.split("-");
    let a = it.next().unwrap().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    let b = it.next().unwrap().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    return Ok((a,b));
}

// Main function
fn main() {
    let filename = "input.txt";
    let result = split_file(filename);
    match result {
        Ok(v) => {
            // Loop through the stored passwords
            for x in v {
                // Split the line => min-max; character; password
                let mut token_iter = x.split_whitespace();
                if let Ok((a, b)) = parse_min_max(token_iter.next().unwrap()) {
                    println!("min={}; max={}", a, b);
                }
                
                /*
                let tokens:Vec<&str> = x.split_whitespace().collect();
                let range = parse_range(tokens[0]);
                */
            }
        },
        Err(e) => {
            println!("*** Error: {}", e);
        }
    }

/*
18-19 p: fvpkgfkfjgwllqwhrjd
12-13 v: kvvvbmdvvvvvvcvvvv
3-5 m: mdmkmvhszpjcxl
*/
}
