use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

// Split file into vector of strings
//  + path:     filepath
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
//  + range:    input string for parsing 
fn parse_min_max(s: &str) -> Result<(i32, i32), std::io::Error> {
    let mut it = s.split("-");
    let a = it.next().unwrap().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    let b = it.next().unwrap().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    return Ok((a,b));
}

// Part-1: Returns true if password contains at least limit_min amount of seek_char but not more than limit_max
//  + passwd:       Password string
//  + seek_char:    Character being searched from the password
//  + limit_min:    Min amount of seek_char to be found from password
//  + limit_max:    Max amount of seek_char to be found from passowrd
fn check_validity1(passwd: &str, seek_char: char, limit_min: i32, limit_max: i32) -> bool {
    let c = passwd.matches(seek_char).count() as i32;
    if c >= limit_min && c <= limit_max {
        //println!("{} contains {} of {} [{},{}]", passwd, c, seek_char, limit_min, limit_max);
        return true;
    }
    return false;
}

// Part-2: Returns true if password has seek_char at either pos1 or pos2 but not at both at the same time
//  + passwd:       Password string
//  + seek_char:    Character being searched from the password
//  + pos1:         1st position for seek_char
//  + pos2:         2nd position for seek_char
fn check_validity2(passwd: &str, seek_char: char, pos1: usize, pos2: usize) -> bool {
    let a = passwd.chars().nth(pos1).unwrap();
    let b = passwd.chars().nth(pos2).unwrap();
    if a != b && (a == seek_char || b == seek_char) {
        return true;
    }
    return false;
}

// Main function
fn main() {
    let filename = "input.txt";
    let result = split_file(filename);
    match result {
        Ok(lines) => {
            let mut valid_passwords_1 = 0;
            let mut valid_passwords_2 = 0;

            // Loop through the stored passwords
            for line in lines {
                // Split the line tokens where [0]="min-max" [1]="character" [2]="password string"
                let mut token_iter = line.split_whitespace();
                if let Ok((a, b)) = parse_min_max(token_iter.next().unwrap()) {
                    let seek_char = token_iter.next().unwrap().chars().nth(0).unwrap();
                    let passwd = token_iter.next().unwrap();

                    // Count valid passwords for Part-1
                    if check_validity1(passwd, seek_char, a, b) {
                        valid_passwords_1 += 1;
                    }

                    // Count valid passwords for Part-2
                    if check_validity2(passwd, seek_char, (a-1) as usize, (b-1) as usize) {
                        valid_passwords_2 += 1;
                    }
                }
            }
            println!("Part-1: Total number of valid passwords is {}", valid_passwords_1);
            println!("Part-2: Total number of valid passwords is {}", valid_passwords_2);
        },
        Err(e) => {
            println!("*** Error: {}", e);
        }
    }
}