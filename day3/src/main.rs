use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename);
    let br = BufReader::new(file.unwrap());

    let mut tree_count = 0;
    let mut pos = 0;
    for line in br.lines() {
        let line: String = line.unwrap();
        let divisor = line.len();

        if line.chars().nth(pos).unwrap() == '#' {
            tree_count += 1;
        }
        pos = (pos + 3) % divisor;
    }
    println!("Num of encountered trees is {}", tree_count);
}
