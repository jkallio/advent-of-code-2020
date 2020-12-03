use std::fs::File;
use std::io::{BufRead, BufReader};

// Traverse through the input file
//  + filename:     Name of the file to be traversed
//  + right:        Number of steps to the right in each iteration
//  + down:         Number of steps down in each iteration
//  -> Returns number of trees '#' encountered while traversing
fn traverse(filename: &str, right: usize, down: usize) -> i64 {
    let file = File::open(filename);
    let mut tree_count = 0;
    let mut pos = 0;
    let br = BufReader::new(file.unwrap());
    for (index, buf) in br.lines().enumerate() {  
        if index % down == 0 {
            let line = buf.unwrap();
            let divisor = line.len();
            if line.chars().nth(pos).unwrap() == '#' {
                tree_count += 1;
            }
            pos = (pos + right) % divisor;
        }
    }
    return tree_count;
}

// Main function
fn main() {
    let filename = "input.txt";

    let a = traverse(filename, 1, 1);
    let b = traverse(filename, 3, 1);
    let c = traverse(filename, 5, 1);
    let d = traverse(filename, 7, 1);
    let e = traverse(filename, 1, 2);

    println!("Part-1: Num of encountered trees is {}", b);
    println!("Part-2: {} * {} * {} * {} * {} = {}", a, b, c, d, e, (a * b * c * d * e));
}
