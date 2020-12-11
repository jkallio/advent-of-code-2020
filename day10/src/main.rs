
fn count_joltage_diff(vec:&Vec<i64>) -> Vec<i64> {
    let mut diffs = Vec::new();
    let mut one_count = 0;
    let mut three_count = 0;
    for adapters in vec.windows(2) {
        match adapters[1] - adapters[0] {
            1 => {
                one_count += 1;
                diffs.push(1);
            }
            3 => {
                three_count += 1;
                diffs.push(3);
            }
            _ => {
                panic!("invalid joltage");
            }
        }
    }
    println!("Jolt differences multplied = {}", one_count * three_count);
    diffs
}

fn main() {
    let input = "input.txt";
    let mut vec = file_utils::read_to_i64_vec(input).unwrap();
    vec.push(0); // Starting from 0 jolts
    vec.sort_unstable();
    vec.push(vec.last().unwrap() + 3); // Internal adapter is 3 higher than highest adapter in the bag

    let diff = count_joltage_diff(&vec);

    let mut one_series = Vec::new();
    let mut i = 0;
    while i < diff.len() {
        if diff[i] == 1 {
            let mut count = 0;
            while diff[i] == 1 {
                count += 1;
                i += 1;
            }
            one_series.push(count);
        }
        i += 1;
    }

    let mut vars:i64 = 1;
    for z in one_series {
        match z {
            1 => { vars *= 1; }
            2 => { vars *= 2; }
            3 => { vars *= 4; }
            4 => { vars *= 7; }
            _ => { panic!(""); }
        }
    }
    println!("{}", vars);
}
