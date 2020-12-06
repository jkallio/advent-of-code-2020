mod jk;
use jk::file_reader;
use std::collections::HashMap;

// Counts the same, and unique answers for each group and adds them together as a total sum
// -> Returns the total sum of unique and the same answers of all groups
fn count_group_answers(input: &Vec<String>) -> (i32, i32) {
    let mut tot_unique_answers:i32 = 0;
    let mut tot_same_answers:i32 = 0;

    let mut group_answers = HashMap::new();
    let mut group_size:i32 = 0;
    for (i, line) in input.iter().enumerate() {
        
        // Each line in the input vector represents the answers of one person in a group
        if line.len() > 0 {
            group_size += 1;
            for c in line.chars() {
                let mut count = 1;
                if let Some(current) = group_answers.get(&c) {
                    count += current;
                }
                group_answers.insert(c, count);
            }
        }

        // Groups are separated by blank line. So after blank line encountered increment the total answer count variables.
        // Also remember to calculate also the last group's answers
        if line.len() == 0 || i == input.len()-1 {

            // Count the same answers within a group
            let mut same_answers = 0;
            for (_, v) in group_answers.iter() {
                if *v as i32 == group_size {
                    same_answers += 1;
                }
            }

            // Increment the total counters
            tot_same_answers += same_answers;
            tot_unique_answers += group_answers.len() as i32;
            
            // Reset the Hash Map and the Group size counter (prepare for the next group)
            group_answers = HashMap::new();
            group_size = 0;
        }
    }
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
