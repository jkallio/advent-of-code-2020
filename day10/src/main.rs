fn get_next_slice(vec:&Vec<i64>) -> Option<Vec<i64>> {


    None
}

fn count_joltage_diff(vec:&Vec<i64>) -> i64 {
    let mut count_ones = 0;
    let mut count_threes = 0;
    for adapters in vec.windows(2) {
        match adapters[1] - adapters[0] {
            1 => {
                count_ones += 1;
            }
            3 => {
                count_threes += 1;
            }
            _ => {
                panic!("invalid joltage");
            }
        }
    }
    return count_ones * count_threes;
}

fn main() {
    let input = "input.txt";
    let mut vec = file_reader::read_to_i64_vec(input).unwrap();
    vec.push(0); // Starting from 0 jolts
    vec.sort_unstable();
    vec.push(vec.last().unwrap() + 3); // Internal adapter is 3 higher than highest adapter in the bag

    let joltage_diff = count_joltage_diff(&vec);
    println!("Jolt differences multplied = {}", joltage_diff);
}
