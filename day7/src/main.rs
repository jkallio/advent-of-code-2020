mod jk;
use jk::file_reader;
use std::collections::HashMap;

// Parse the bag contents
fn parse_bag_contents(input: &str) -> Result<Vec<(i64, String)>, &'static str> {
    let input_parts: Vec<&str> = input.split(',').collect();

    let mut contents: Vec<(i64, String)> = Vec::new();
    for bag in input_parts {

        let bag_parts: Vec<&str> = bag.trim_start().split_whitespace().collect();
        assert_eq!(bag_parts.len() == 4, true);

        // The bag description consists of <amount> <colour> "bag(s)" => e.g. "5 wavy cyan bags"
        let amount:i64 = bag_parts[0].parse::<i64>().unwrap();
        let colour_key = format!("{} {}", bag_parts[1], bag_parts[2]);
        assert_eq!(bag_parts[3].contains("bag"), true);
        contents.push((amount, colour_key));
    }
    return Ok(contents);
}

// Parse a single line of input.txt
fn parse_input_line(input: &str) -> Result<(String, Vec<(i64, String)>), &'static str> {
    let input_parts: Vec<&str> = input.split(" bags contain ").collect();
    assert_eq!(input_parts.len() == 2, true); // Invalid input

    // Left side of the input string contains the key for the HashMap
    if let Some(k) = input_parts.first() {
        let key = String::from(*k);

        // Right side of the input string contains comma separated list of content
        if let Some(value) = input_parts.last() {
            let mut bag_content:Vec<(i64, String)> = Vec::new();
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

// Recursive function that goes through the nested list of bags
fn contains_shiny_gold(key:&String, bags:&HashMap<String, Vec<(i64, String)>>) -> bool {
    if let Some(others) = bags.get(key) {
        for other in others {  
            let key = &other.1;
            if key.contains("shiny gold") {
                return true;
            }
            else {
                if contains_shiny_gold(&key, bags) == true {
                    return true;
                }
            }
        }
    }
    return false;
}

// Recursive function that counts all the bags inside of a bag
fn count_bags_inside(key:&String, bags:&HashMap<String, Vec<(i64, String)>>) -> i64 {
    let mut bag_count:i64 = 0;
    if let Some(others) = bags.get(key) {
        for other in others {
            bag_count += other.0;
            bag_count += other.0 * count_bags_inside(&other.1, &bags);
        }
    }
    bag_count
}

fn main() {
    let filename = "input.txt";
    if let Ok(input_vec) = file_reader::read_to_vec(filename) {

        // Each bag contains at least one other bag (stored as list of keys)
        let mut bags = HashMap::<String, Vec<(i64, String)>>::new();
        for line in input_vec {
            if let Ok(bag) = parse_input_line(&line) {
                bags.insert(bag.0, bag.1);
            }
        }

        // Part-1: Count all the bags that eventually holds a shiny gold bag
        let mut bags_containing_shiny_gold = 0;
        for key in bags.keys() {
            if contains_shiny_gold(&key, &bags) {
                bags_containing_shiny_gold += 1;
            }
        }

        // Part-2: Count all the bags your shiny gold bag must have
        let bags_inside_shiny_gold = count_bags_inside(&String::from("shiny gold"), &bags);

        
        println!("Total bags containing shiny gold is {}", bags_containing_shiny_gold);
        println!("Shiny gold bag contains {} bags", bags_inside_shiny_gold);
    }
}
