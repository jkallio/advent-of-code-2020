/// Consecutive one jolt differences increase the number of ways adapters can be connected
/// e.g. ...(1)-(4)-(5)-(6)-(9) has two consecutive one jolt differencse => it adds another permumation
///      -> (1)-(4)-----(6)-(9)
///
/// e.g. (1)-(4)-(5)-(6)-(7)-(10) has three consecutive one jolt differences => it has 4 permutations
///   -> (1)-(4)-(5)-----(7)-(10)
///   -> (1)-(4)-----(6)-(7)-(10)
///   -> (1)-(4)---------(7)-(10)
///
/// e.g. (1)-(4)-(5)-(6)-(7)-(8)-(11) has four consecutive one jolt differences => it has 7 permutations
///   -> (1)-(4)-(5)-----(7)-(8)-(11)
///   -> (1)-(4)-----(6)-(7)-(8)-(11)
///   -> (1)-(4)---------(7)-(8)-(11)
///   -> (1)-(4)-(5)-(6)-----(8)-(11)
///   -> (1)-(4)-----(6)-----(8)-(11)
///   -> (1)-(4)-(5)---------(8)-(11)
fn main() {
    let input = "input.txt";
    let mut vec = file_utils::read_to_i64_vec(input).unwrap();
    vec.push(0); // Starting from 0 jolts
    vec.sort_unstable();
    vec.push(vec.last().unwrap() + 3); // Internal adapter is 3 higher than highest adapter in the bag

    let mut one_count = 0; // Number of one jolt differences
    let mut three_count = 0; // Number of three jolt differences
    let mut consecutive_one_count = 0; // Counts consecutive one jolt differences

    let mut permutations: i64 = 1; // Counts the total number of ways adapters can be connected

    for adapters in vec.windows(2) {
        match adapters[1] - adapters[0] {
            1 => {
                one_count += 1;
                consecutive_one_count += 1;
            }
            3 => {
                three_count += 1;

                // Consecutive one jolt differences increase permutations (see examples on top)
                match consecutive_one_count {
                    1 => {
                        permutations *= 1;
                    }
                    2 => {
                        permutations *= 2;
                    }
                    3 => {
                        permutations *= 4;
                    }
                    4 => {
                        permutations *= 7;
                    }
                    _ => {}
                }
                consecutive_one_count = 0; // Reset the counter
            }
            _ => {
                panic!("invalid joltage");
            }
        }
    }

    println!("Jolt differences multplied = {}", one_count * three_count);
    println!("Total number of permutations = {}", permutations);
}
