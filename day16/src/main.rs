use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

type RangeMap = HashMap<String, ValueRange>;
type TicketList = Vec<Vec<i32>>;

#[derive(Debug)]
struct ValueRange {
    lower_min: i32,
    lower_max: i32,
    upper_min: i32,
    upper_max: i32,
}

impl ValueRange {
    fn value_in_range(&self, v: i32) -> bool {
        (v >= self.lower_min && v <= self.lower_max) || (v >= self.upper_min && v <= self.upper_max)
    }
}

// Parse input file
fn parse_input_file(
    path: &str,
    ranges: &mut RangeMap,
    tickets: &mut TicketList,
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
                        // 1st section consists of pair of value ranges (e.g. 28-184 or 203-952)
                        let mut parts_iter = line.split(':');
                        let key = parts_iter.next().unwrap();

                        let re = Regex::new(r"[0-9]+-[0-9]+").unwrap();
                        let rngs = re.find_iter(&line).map(|m| m.as_str());

                        let mut value_range = ValueRange {
                            lower_max: 0,
                            lower_min: 0,
                            upper_max: 0,
                            upper_min: 0,
                        };

                        for (i, r) in rngs.enumerate() {
                            let mut r_iter = r.split('-');
                            let min_value = r_iter.next().unwrap().parse().unwrap();
                            let max_value = r_iter.next().unwrap().parse().unwrap();
                            match i {
                                0 => {
                                    value_range.lower_min = min_value;
                                    value_range.lower_max = max_value;
                                }
                                1 => {
                                    value_range.upper_min = min_value;
                                    value_range.upper_max = max_value;
                                }
                                _ => {
                                    panic!("Invalid range");
                                }
                            }
                        }
                        ranges.insert(String::from(key), value_range);
                    }

                    // 2nd section consist of values in your own ticket (comma separated)
                    // 3rd section consists of values in your neighbors tickets
                    1 | 2 => {
                        // Own Nearby Ticket
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

// Returns true if ticket is included at least in one range
fn is_ticket_valid(ticket_values: &[i32], ranges: &RangeMap) -> bool {
    for value in ticket_values {
        let mut is_valid = false;
        for range in ranges.values() {
            if range.value_in_range(*value) {
                is_valid = true;
                break;
            }
        }
        if !is_valid {
            return false;
        }
    }
    true
}

// Returns true if each of ticket[i] value is within given range
fn is_ticket_index_in_range(i: usize, range: &ValueRange, tickets: &[Vec<i32>]) -> bool {
    for ticket in tickets {
        let value = ticket.get(i).unwrap();
        if !range.value_in_range(*value) {
            return false;
        }
    }
    true
}

fn main() {
    let input = "input.txt";
    let mut ranges = RangeMap::new();
    let mut tickets = TicketList::new();

    if parse_input_file(input, &mut ranges, &mut tickets).is_ok() {
        // Remove invalid tickets
        tickets.retain(|t| is_ticket_valid(&t, &ranges));

        // Your own ticket is the first one in the list
        let your_ticket = tickets.first().unwrap().clone();

        // Collect range-index for each ticket value slot
        let mut range_candidates = Vec::<(usize, Vec<String>)>::new();
        for i in 0..ranges.len() {
            let mut vec = vec![];
            for range in &ranges {
                if is_ticket_index_in_range(i, &range.1, &tickets) {
                    vec.push(String::from(range.0));
                }
            }
            range_candidates.push((i, vec));
        }

        // Sort starting from smallest list
        range_candidates.sort_by_key(|t| t.1.len());

        // Eliminate the fields one by one
        let mut ticket_values = HashMap::<String, i32>::new();
        loop {
            // Find a list that contains only one possible filed
            let mut key: String = "".to_string();
            for it in range_candidates.iter() {
                if it.1.len() == 1 {
                    key = String::from(it.1.first().unwrap());
                    let value = *your_ticket.get(it.0).unwrap();
                    ticket_values.insert(String::from(&key), value); // Store the value from your ticket that matches the index slot
                    break;
                }
            }
            if !key.is_empty() {
                // Eliminate the found field key from other lists
                for it in range_candidates.iter_mut() {
                    it.1.retain(|k| !k.contains(&key));
                }
            } else {
                break;
            }
        }

        // Final answer is the result of all the fields that starts with "departure"
        let mut multiplication_result: i64 = 1;
        for value in ticket_values {
            if value.0.starts_with("departure") {
                multiplication_result *= value.1 as i64;
            }
        }
        println!("Final answer is {}", multiplication_result);
    }
}
