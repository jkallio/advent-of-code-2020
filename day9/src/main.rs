/// Returns true if at least one pair of values are found from `vec` that have the sum of `sum_target`
///
/// # Arguments
/// * `vec` input from where to search the addends.
/// * `sum_target` target value for the sum.
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

/// Returns first i64 in `vec` that does not follow the rule where each number must be the sum of
/// any two earlier number within a window of 25.
/// 
/// # Arguments
/// * `vec` input vector containing the number pool
fn find_first_invalid_number(vec: &Vec<i64>) -> Option<i64> {
    for slice in vec.windows(26) {        
        if !find_addend_pair(&slice.to_vec(), slice[25]) {
            return Some(slice[25]);
        }
    }
    None
}

/// Returns the "encryption weakness" which is the sum of smallest and largest values in the
/// window that sums up to `target_sum`
/// 
/// # Arguments
/// `vec` input vector containing the number pool
/// `target_sum` target sum that we're trying to find
fn find_encryption_weakness(vec: &Vec<i64>, target_sum:i64) -> Option<i64> {
    let mut window_size = 2;
    loop {
        for slice in vec.windows(window_size) {
            let sum:i64 = slice.iter().sum();
            if sum == target_sum {
                return Some(slice.iter().min().unwrap() + slice.iter().max().unwrap());
            }
        }
        
        window_size += 1;
        if window_size >= vec.len() {
            return None;
        }
    }
}

/// Main function
fn main() {
    let input = "input.txt";
    let vec = file_reader::read_to_i64_vec(input);
    match vec {
        Ok(vec) => {
            // Part-1:
            let invalid_number = find_first_invalid_number(&vec).unwrap();
            println!("First value to not to follow the rule is {}", invalid_number);

            // Part-2:
            let encryption_weakness = find_encryption_weakness(&vec, invalid_number).unwrap();
            println!("Encryption weakness is {}", encryption_weakness);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
