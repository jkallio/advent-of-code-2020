mod jk;
use jk::file_reader;
use std::collections::HashMap;

fn count_group_answers(input: &Vec<String>) -> (i32, i32) {
    let mut tot_unique_answers:i32 = 0;
    let mut tot_same_answers:i32 = 0;

    let mut group_answers = HashMap::new();
    let mut group_size:i32 = 0;
    for line in input {
        if line.len() == 0 {
            let mut same_answers = 0;
            for (_, v) in group_answers.iter() {
                if *v as i32 == group_size {
                    same_answers += 1;
                }
            }
            tot_same_answers += same_answers;
            tot_unique_answers += group_answers.len() as i32;
            group_answers = HashMap::new();
            group_size = 0;
        }
        else {
            group_size += 1;
            for c in line.chars() {
                let mut count = 1;
                if let Some(current) = group_answers.get(&c) {
                    count += current;
                }
                group_answers.insert(c, count);
            }
        }
    }
    let mut same_answers = 0;
    for (_, v) in group_answers.iter() {
        if *v as i32 == group_size {
            same_answers += 1;
        }
    }
    tot_same_answers += same_answers;
    tot_unique_answers += group_answers.len() as i32;
    return (tot_unique_answers, tot_same_answers);
}

fn main() {
    let filename = "input.txt";
    if let Ok(vec) = file_reader::read_to_vec(filename) {
        let (unique, same) = count_group_answers(&vec);
        println!("Total unique answer count is {}", unique);
        println!("Total same answer count is {}", same);
    }
}
