use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type RuleMap = HashMap<i32, String>;

// Parse input file into separate `rules` HashMap and `messages` list
fn parse_input_file(path: &str, rules: &mut RuleMap, messages: &mut Vec<String>) {
    let file = File::open(path).unwrap();
    let br = BufReader::new(file);
    let mut section = 0;
    for line in br.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            section += 1;
        } else {
            match section {
                0 => {
                    // First section contains the rules
                    let mut it = line.split(": ");
                    let rule_num = it.next().unwrap().parse::<i32>().unwrap();
                    let rule = it.next().unwrap();
                    rules.insert(rule_num, String::from(rule));
                }

                1 => {
                    // Second section contains the messages
                    messages.push(line);
                }

                _ => {
                    panic!("Invalid section!");
                }
            }
        }
    }
}

// Solves the given rule id recursively until "a" or "b" is found.
fn recursive_solve(rules: &RuleMap, rule_id: i32, mut inf_count:i32) -> String {
    let mut solved = String::new();
    if let Some(rule) = &rules.get(&rule_id) {
        solved = String::from(*rule);
        if *rule == "\"a\"" || *rule == "\"b\"" {
            solved.retain(|c| c != '"');
        } else {
            let inject_parentheses = rule.contains('|');
            loop {
                let re = Regex::new(r"[0-9]+").unwrap();
                let search_str = String::from(&solved);
                if let Some(m) = re.find(&search_str) {
                    let id = &solved[m.start()..m.end()].parse::<i32>().unwrap();

                    if *id == rule_id {
                        inf_count += 1
                    }

                    if inf_count < 10 {
                        let substr = recursive_solve(&rules, *id, inf_count);
                        solved.replace_range(m.start()..m.end(), &substr);
                    }
                    else {
                        solved.replace_range(m.start()-1..m.end(), "");
                    }            
                } else {
                    break;
                }
            }
            if inject_parentheses {
                solved = format!("({})", solved);
            }
        }
    }
    assert_eq!(solved.is_empty(), false);
    solved
}

fn main() {
    let input = "input2.txt";
    let mut rules = RuleMap::new();
    let mut messages = vec![];
    parse_input_file(&input, &mut rules, &mut messages);

    let mut the_rule = recursive_solve(&rules, 0, 0);

    // Trim the search string and add ´line begin` and `line end` rules
    the_rule.retain(|c| !c.is_whitespace());
    the_rule = format!("^({})$", the_rule);

    // Count the total amount of messages that match the constructed regular experssion
    let re = Regex::new(&the_rule).unwrap();
    let mut match_count = 0;
    for msg in messages {
        if re.is_match(&msg) {
            println!("{}", msg);
            match_count += 1;
        }
    }
    println!("Total matches = {}", match_count);
}
