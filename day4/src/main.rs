use std::fs::File;
use std::io::{BufRead, BufReader};
mod passport;
use passport::Passport;
//use passport::Validator;

fn parse_line(line: &str) -> Passport {
    let mut pass = Passport::new();
    let params = line.split_whitespace();
    for it in params {
        pass.set(it);
    }
    return pass;
}


fn parse_file(path: &str) -> Result<Vec<Passport>, std::io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut vec = Vec::new();
    let mut pass = Passport::new();
    for line in br.lines() {
        let line = line?;

        // Create new passport (separated by blank line)
        if line.len() == 0 {
            vec.push(pass);
            pass = Passport::new();
        }
        else {
            pass.merge(&parse_line(&line));    
        }
    }
    vec.push(pass);

    return Ok(vec);
}



// Main function
fn main() {
    let filename = "input.txt";
    let result = parse_file(filename);
    let mut valid_count = 0;
    let mut invalid_count = 0;
    match result {
        Ok(vec) => {
            for pass in vec {
                pass.print_all();

                if pass.is_valid() {
                    valid_count += 1;
                }
                else {
                    invalid_count += 1;

                }
            }
        }
        Err(e) => {
            panic!("Error parsing file: {}", e);
        }
    }

    // Guess: 164 is too low
    // Guess: 246 is too low
    // Correct answer was 247
    println!("Total {} passports are valid!", valid_count);
    println!("Total {} passports are invalid!", invalid_count);
    
}
