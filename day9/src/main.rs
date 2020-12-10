/// Returns true if at least one pair of values are found from `vec` that have the sum of `sum_target`
///
/// # Arguments
/// * `vec` input from where to search the addends.
/// * `sum_target` target value for the sum.
///
/// # Examples
/// ```
/// let vec = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
/// let sum_target = 403; // = 95 + 309
/// assert_eq!(find_addend_pair(vec, sum_target), true);
/// ```
fn find_addend_pair(vec: &Vec<i64>, sum_target: i64) -> bool {
    let max_index = vec.len();
    for (i, a) in vec.iter().enumerate() {
        for (j, b) in vec.iter().enumerate() {
            if i < max_index && j < max_index && a + b == sum_target {
                return true;
            }
        }
    }
    false
}



/// Main function
fn main() {
    let input = "input.txt";
    let vec = file_reader::read_to_i64_vec(input);
    match vec {
        Ok(vec) => {
            // Part-1 --> 
            let mut invalid_number:i64 = 0;
            for slice in vec.windows(26) {
                let sum_target:i64 = *slice.last().unwrap();
                if !find_addend_pair(&(&slice[..25]).to_vec(), sum_target) {
                    invalid_number = sum_target;
                    break;
                }
            }
            println!("First value to not to follow the rule is {}", invalid_number);

            // Part-2 --> 
            let mut window_size = 2;
            'outer: loop {
                println!("Window size = {}", window_size);
                if window_size < vec.len() {
                    for slice in vec.windows(window_size) {
                        let sum:i64 = slice.iter().sum();
                        if sum == invalid_number {
                            let encryption_weakness = slice.iter().min().unwrap() + slice.iter().max().unwrap();
                            println!("Encryption weakness is {}", encryption_weakness);
                            break 'outer;
                        }
                    }
                }
                else {
                    println!("ERR! Window size exceeds vector size");
                    break;
                }
                window_size += 1;
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
