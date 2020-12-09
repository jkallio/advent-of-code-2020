use file_reader;

fn count_pairs(vec:Vec<i64>) -> i64 {
    let mut count:i64 = 0;
    let target_value = vec.last().unwrap();
    let max_index = vec.len() - 1;
    for (i, a) in vec.iter().enumerate() {
        for (j, b) in vec.iter().enumerate() {
            if i < max_index && j < max_index {
                if a + b == *target_value {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn main() {
    let input = "input.txt";
    let vec = file_reader::read_to_i64_vec(input);
    match vec {
        Ok(vec) => {
            for slice in vec.windows(26) {
                if count_pairs(slice.to_vec()) == 0 {
                    println!("Not found! value is {}", slice.last().unwrap());

                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    println!("End");
}
