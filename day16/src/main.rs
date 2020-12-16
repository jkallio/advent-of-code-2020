use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use regex::Regex;

#[derive(Debug)]
struct ValueRange {
    min:i32,
    max:i32
} 

fn parse_input_file(
    path: &str,
    ranges: &mut HashMap::<String, Vec<ValueRange>>
) -> Result<bool, std::io::Error> {
    
    let file = File::open(path)?;
    let br = BufReader::new(file);
    
    let mut section = 0;
    for line in br.lines() {
        if let Ok(line) = line {
            if line.len() == 0 {
                section += 1;
            }
            else {
                match section {
                    0 => {  // Ranges
                        let mut parts_iter = line.split(":");
                        let key = parts_iter.next().unwrap();

                        let re = Regex::new(r"[0-9]+-[0-9]+").unwrap();
                        let rngs = re.find_iter(&line)
                            .filter_map(|m| Some(m.as_str()))
                            .collect::<Vec<&str>>();

                        let mut value = Vec::<ValueRange>::new();
                        for r in rngs {
                            let mut r_iter = r.split("-");
                            value.push(
                                ValueRange { 
                                    min: r_iter.next().unwrap().parse().unwrap(),
                                    max: r_iter.next().unwrap().parse().unwrap()
                                });
                        }
                        ranges.insert(String::from(key), value);
                    }
                    1 => {  // Your Ticket
                        

                    }
                    2 => {  // Nearby Ticket

                    }
                    _ => {
                        panic!("Invalid Section");
                    }
                }
            }
        }
    }
    Ok(true)
}

fn main() {
    let mut ranges = HashMap::<String, Vec<ValueRange>>::new();

    parse_input_file("input.txt", &mut ranges);
    for (k, v) in ranges {
        println!("{} = {:?}", k, v);
    }
}
