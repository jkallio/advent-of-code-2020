use regex::Regex;
use std::collections::HashMap;

fn parse_mem_instruction(line: &str) -> (i64, i64) {
    let mut iter = line.split(" = ");
    if let Some(left_part) = iter.next() {
        if let Ok(addr) = Regex::new("[0-9]+").unwrap().captures(left_part).unwrap().get(0).map_or("", |m| m.as_str()).parse::<i64>() {
            if let Some(right_part) = iter.next() {
                if let Ok(value) = Regex::new("[0-9]+").unwrap().captures(right_part).unwrap().get(0).map_or("", |m| m.as_str()).parse::<i64>() {
                    return (addr, value);
                }
            }
        }
    }
    (-1, -1)
}

fn parse_mask(line: &str) -> (i64, i64) {
    let iter = line.split(" = ");
    let mut one_mask:i64 = 0;
    let mut zero_mask:i64 = 0;
    if let Some(bit_string) = iter.last() {
        for i in (0..bit_string.len()).rev() {
            match bit_string.chars().nth(i).unwrap() {
                'X' => { }
                '1' => { one_mask += 1 << bit_string.len() - i-1; }
                '0' => { zero_mask += 1 << bit_string.len() - i-1; }
                _ => { panic!("Invalid string"); }
            }
        }
    }
    (zero_mask, one_mask)
}

fn main() { 
    if let Ok(input) = file_utils::read_to_string_vec("input.txt") {
        let mut masks:(i64, i64) = (0, 0);
        let mut mem = HashMap::<i64,i64>::new();
        for line in input {
            if line.starts_with("mem") {
                let m = parse_mem_instruction(&line);
                mem.insert(m.0, (m.1 | masks.1) & !masks.0);
            }
            else if line.starts_with("mask") {
                masks = parse_mask(&line);
            }
        }

        let mut tot_sum:i64 = 0;
        for m in mem {
            println!("mem[{}] = {}", m.0, m.1);
            tot_sum += m.1;
        }
        println!("Total sum of the values in memory is {}", tot_sum);
    }
}
