use std::fs::File;
use std::io::{BufReader, BufRead};

const cmd_list:[&'static str; 3] = ["acc", "jmp", "nop"];

fn main() {
    let filename = "input.txt";

    let file = File::open(filename);
    let mut tree_count = 0;
    let mut pos = 0;
    let br = BufReader::new(file.unwrap());
    for (index, buf) in br.lines().enumerate() {  
        println!("{}", buf);
    }
}
