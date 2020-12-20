use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input_file(path:&str) {
    let file = File::open(path).unwrap();
    let br = BufReader::new(file);
    let mut section = 0;
    for line in br.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            section += 1;
        } else {
            println!("{}: {}", section, line);
            match section {
                0 => {
                    // First section contains the rules
                    
                }

                1 => {
                    // Second section contains the messages
                }

                _ => { panic!("Invalid section!"); }
            }
        }
    }
    
}

fn main() {
    let input = "input.txt";
    parse_input_file(&input);
}
