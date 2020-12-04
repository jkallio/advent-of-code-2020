use std::fs::File;
use std::io::{BufRead, BufReader};
mod passport;
use passport::Passport;
use regex::Regex;

// Parse 32-bit integer from string
//  + value:    Input value as string
// -> Returns i32 if input is valid integer; None otherwise
fn parse_i32(value: &str) -> Option<i32> {
    let i = value.parse::<i32>();
    match i {
        Ok(val) => Some(val),
        Err(_e) => None
    }
}

// Parse (Height) - a number followed by either cm or in: cm[150, 193]; in[59, 67]
//  + value:    Input height as string
// -> Returns i32 if input is valid height; None otherwise
fn parse_height(value:&str) -> Option<i32> {
    if value.contains("in") {
        if let Some(h) = parse_i32(&value[0..2]) {
            if h >= 59 && h <= 76 {
                return Some(h);
            }
        }
    }
    else if value.contains("cm") {
        if let Some(h) = parse_i32(&value[0..3]) {
            if h >= 150 && h <= 193 {
                return Some(h);
            }
        }
    }
    None
}

// Parse (Year) - four digits
//  + value:    Input year as string
//  + at_least: A valid year must be at least given value
//  + at_most:  A valid year must not be over given value
//  -> Returns the year if input is vald integer; None otherwise
fn parse_year(value:&str, at_least:i32, at_most:i32) -> Option<i32> {
    if let Some(y) = parse_i32(value) {
        if y >= at_least && y <= at_most {
            return Some(y);
        }
    }
    None
}

// Parse (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//  + value:    Input string
//  -> Returns the string if valid #rrggbb; None otherwise
fn parse_hair_color(value: &str) -> Option<String> {
    let re = Regex::new(r"#[a-f,0-9]{6}").unwrap();
    if re.is_match(value) {
        return Some(String::from(value));
    }
    None
}

// Parse (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//  + value:    Input string
//  -> Returns the string if matches determined set of values; None otherwise
fn parse_eye_color(value: &str) -> Option<String> {
    if value == "amb" || value == "blu" || value == "brn" || value == "gry" || value == "grn" || value == "hzl" || value == "oth" {
        return Some(String::from(value));
    }
    else {
        return None;
    }
}

// Parse (Passport ID) - a nine-digit number, including leading zeroes.
//  + value:    Input string
//  -> Returns passport ID as integer if valid; None otherwise
fn parse_passport_id(value: &str) -> Option<i32> {
    let re = Regex::new(r"\b[0-9]{9}\b").unwrap();
    if re.is_match(value) {
        return parse_i32(value);
    }
    else {
        return None;
    }
}

// Parse input string into a Passport struct
//  + line:     Input string to be parsed
//  -> Returns Passport struct
fn parse_line(line: &str) -> Passport {
    let mut pass = Passport::new();
    let params = line.split_whitespace();
    for it in params {
        let mut parts = it.split(":");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        match key {
            "byr" => pass.byr = parse_year(value, 1920, 2002),
            "iyr" => pass.iyr = parse_year(value, 2010, 2020),
            "eyr" => pass.eyr = parse_year(value, 2020, 2030),
            "hgt" => pass.hgt = parse_height(value),
            "hcl" => pass.hcl = parse_hair_color(value),
            "ecl" => pass.ecl = parse_eye_color(value),
            "pid" => pass.pid = parse_passport_id(value),
            "cid" => pass.cid = parse_i32(value),
            _ => panic!("Unknwon param name!")
        }
    }
    return pass;
}

// Parse passport file 
// - Passport fields are separated by whitespace (space / linebreak)
// - Passports are separated by empty line
//  + path:     Path to the input file
//  -> Returns list of Passports; Error in case of error
fn parse_file(path: &str) -> Result<Vec<Passport>, std::io::Error> {
    let file = File::open(path)?;   // Early exit in case of IO error
    let br = BufReader::new(file);
    let mut vec = Vec::new();
    let mut pass = Passport::new();
    for line in br.lines() {
        let line = line?; // Early exit in case of IO error

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
    Ok(vec)
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
    println!("Total {} passports are valid!", valid_count);
    println!("Total {} passports are invalid!", invalid_count);
}
