use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

type RuleMap = HashMap::<i32,String>;

// Parse input file into separate `rules` HashMap and `messages` list
fn parse_input_file(
    path: &str,
    rules: &mut RuleMap,
    messages: &mut Vec<String>) {

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

                _ => { panic!("Invalid section!"); }
            }
        }
    }
    
}

fn remove_whitespace(s:&str) -> String {
    let mut trimmed = String::from(s);
    trimmed.retain(|c| !c.is_whitespace());
    trimmed
}

fn parse_rule(rules:&RuleMap, rule:&str) -> String {
    let mut new_rule = String::from(rule);
    new_rule.retain(|c| c != '\"');

    if new_rule != "a" && new_rule != "b" {
       
        //let mut rule_list = vec![];
        for (i, parts) in rule.split('|').enumerate() {
           
            //let mut vec = vec![];
            for it in parts.split(' ') {
                let rule_ref = remove_whitespace(&it);
                if !rule_ref.is_empty() {
                    let rule = parse_rule(&rules, rules.get(&rule_ref.parse::<i32>().unwrap()).unwrap());
                    println!("{}", rule);   
                }
                
            }
        }
    }
    new_rule
}

fn main() {
    let input = "input.txt";
    let mut rules = RuleMap::new();
    let mut messages = vec![];
    parse_input_file(&input, &mut rules, &mut messages);

    let mut the_rule = String::from(rules.get(&0).unwrap());
    the_rule = parse_rule(&rules, &mut the_rule);
    println!("{}", the_rule);

    /*
    for (k,v) in rules {
        println!("{} = {}", k, v);
    }

    for m in messages {
        println!("{}", m);
    }
    */
}
