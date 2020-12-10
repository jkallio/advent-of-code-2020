use file_reader;

/// Returns `true` if given vector contains at least one pair of
/// addends that sum up to the value found from the last position
/// of the input vector.
/// 
/// # Arguments
/// * `vec` input vector that contains the values where to search the addends.
/// 
/// # Examples
/// let vec = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
/// 
fn find_addend_pair(vec:Vec<i64>, sum_target:i64) -> bool {
    let max_index = vec.len();
    for (i, a) in vec.iter().enumerate() {
        for (j, b) in vec.iter().enumerate() {
            if i < max_index && j < max_index {
                if a + b == sum_target {
                    return true;
                }
            }
        }
    }
    return false;
}

/// Main function
fn main() {
    let input = "input.txt";
    let vec = file_reader::read_to_i64_vec(input);
    match vec {
        Ok(vec) => {
            for slice in vec.windows(26) {
                if find_addend_pair((&slice[..25]).to_vec(), slice[25]) == false {
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
