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
        Ok(lines) => {
            let mut valid_passwords = 0;
            // Loop through the stored passwords
            for line in lines {
                // Split the line tokens where [0]="min-max" [1]="character" [2]="password string"
                let mut token_iter = line.split_whitespace();
                if let Ok((a, b)) = parse_min_max(token_iter.next().unwrap()) {
                    let seek_char = token_iter.next().unwrap().chars().nth(0).unwrap();
                    let passwd = token_iter.next().unwrap();

                    // Count occurances of seek_char in passwd
                    let c = passwd.matches(seek_char).count() as i32;
                    if c >= a && c <= b {
                        valid_passwords += 1;
                    }
                }
            }
            println!("Total number of valid passwords is {}", valid_passwords);
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
