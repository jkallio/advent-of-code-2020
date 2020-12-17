use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

#[derive(Debug)]
struct ValueRange {
    min: i32,
    max: i32,
}

impl ValueRange {
    fn value_in_range(&self, v: i32) -> bool {
        v >= self.min && v <= self.max
    }
}

fn parse_input_file(
    path: &str,
    ranges: &mut HashMap<String, Vec<ValueRange>>,
    tickets: &mut Vec<Vec<i32>>,
) -> Result<bool, std::io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut section = 0;
    for line in br.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                section += 1;
            } else {
                match section {
                    0 => {
                        // Ranges
                        let mut parts_iter = line.split(':');
                        let key = parts_iter.next().unwrap();

                        let re = Regex::new(r"[0-9]+-[0-9]+").unwrap();
                        let rngs = re.find_iter(&line).map(|m| m.as_str());
                        
                        let mut value = Vec::<ValueRange>::new();
                        for r in rngs {
                            let mut r_iter = r.split('-');
                            value.push(ValueRange {
                                min: r_iter.next().unwrap().parse().unwrap(),
                                max: r_iter.next().unwrap().parse().unwrap(),
                            });
                        }
                        ranges.insert(String::from(key), value);
                    }
                    1 => { // Your Ticket
                    }
                    2 => {
                        // Nearby Ticket
                        if line.contains(',') {
                            let parts_iter = line.split(',');
                            let mut values = Vec::<i32>::new();
                            for value in parts_iter {
                                let n: Result<i32, std::io::Error> = value
                                    .parse()
                                    .map_err(|e| Error::new(ErrorKind::InvalidData, e));
                                values.push(n?);
                            }
                            tickets.push(values);
                        }
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

fn check_ticket_validity(ranges: &HashMap<String, Vec<ValueRange>>, ticket: &[i32]) -> bool {
    for value in ticket {
        let mut is_valid = false;
        'outer: for category in ranges {
            for rng in category.1 {
                if rng.value_in_range(*value) {
                    is_valid = true;
                    break 'outer;
                }
            }
        }
        if !is_valid {
            return false;
        }
    }
    true
}

fn main() {
    let input = "input.txt";
    let mut ranges = HashMap::<String, Vec<ValueRange>>::new();
    let mut tickets = Vec::<Vec<i32>>::new();

    if parse_input_file(input, &mut ranges, &mut tickets).is_ok() {

        // Remove invalid tickets
        tickets.retain(|t| check_ticket_validity(&ranges, &t));
    }
}
