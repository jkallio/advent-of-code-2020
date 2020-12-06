mod jk;
use jk::file_reader;
use std::collections::HashMap;

fn main() {
    let filename = "input.txt";
    if let Ok(vec) = file_reader::read_to_vec(filename) {
        let mut answers = HashMap::new();
        let mut total_count = 0;
        for line in vec {
            if line.len() == 0 {
                // Clear HashMap for new group
                total_count += answers.len();
                println!("Num of answers is {}", answers.len());
                answers = HashMap::new();
            }
            else {
                for c in line.chars() {
                    let mut count = 1;
                    if let Some(current) = answers.get(&c) {
                        count += current;
                    }
                    answers.insert(c, count);  
                }
            }
        }
        total_count += answers.len(); // Also add the last group answers
        
        println!("Total count is {}", total_count);
    }
}
