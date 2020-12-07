mod jk;
use jk::file_reader;
use std::collections::HashMap;

fn parse_bag_contents(input: &str) -> Result<Vec<(i32, String)>, &'static str> {
    let input_parts: Vec<&str> = input.split(',').collect();

    let mut contents: Vec<(i32, String)> = Vec::new();
    for bag in input_parts {

        let bag_parts: Vec<&str> = bag.trim_start().split_whitespace().collect();
        assert_eq!(bag_parts.len() == 4, true);

        // The bag description consists of <amount> <colour> "bag(s)" => e.g. "5 wavy cyan bags"
        let amount:i32 = bag_parts[0].parse::<i32>().unwrap();
        let colour_key = format!("{} {}", bag_parts[1], bag_parts[2]);
        assert_eq!(bag_parts[3].contains("bag"), true);
        contents.push((amount, colour_key));
    }
    return Ok(contents);
}

fn parse_input_line(input: &str) -> Result<(&str, Vec<(i32, String)>), &'static str> {
    let input_parts: Vec<&str> = input.split(" bags contain ").collect();
    assert_eq!(input_parts.len() == 2, true); // Invalid input

    // Left side of the input string contains the key for the HashMap
    if let Some(key) = input_parts.first() {
        
        // Right side of the input string contains comma separated list of content
        if let Some(value) = input_parts.last() {
            let mut bag_content:Vec<(i32, String)> = Vec::new();
            if !value.contains("no other") {
                if let Ok(b) = parse_bag_contents(value) {
                    bag_content = b;
                }
            }
            return Ok((key, bag_content));
        }
    }
    panic!("Failed to parse input: \"{}\"", input);
}

fn main() {
    let filename = "input.txt";
    if let Ok(vec) = file_reader::read_to_vec(filename) {

        // Each bag contains at least one other bag (stored as list of keys)
        let _bags = HashMap::<&str, Vec<String>>::new();

        for line in vec {
            if let Ok(bag) = parse_input_line(&line) {
                println!("'{}' contents: ", bag.0);
                for b in bag.1 {
                    println!("   {}: {}", b.0, b.1);
                }
            }

            /*
            for part in parts {
                println!("{}", part);
            }

            // Regex
            let seperator = Regex::new(r"contain [0-9]").expect("Invalid regex");
            let result = seperator.split(s);
            for res in result {
                println!("{}", res);
            }
            */
        }
    }
}
