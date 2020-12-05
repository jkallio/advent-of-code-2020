mod jk;
use jk::file_reader;
use std::io::Error;

fn parse_row_and_seat(input: &str) -> Result<(i32, i32), std::io::Error> {
    let mut row = (0, 127);
    let mut col = (0, 7);
    
    for a in input.chars() {
        let rows = (row.1 - row.0 + 1) / 2;
        let cols = (col.1 - col.0 + 1) / 2;
        match a {
            'F' => {    row.1 = row.1 - rows;   }
            'B' => {    row.0 = row.0 + rows;   }
            'L' => {    col.1 = col.1 - cols;   }
            'R' => {    col.0 = col.0 + cols;   }
            _ => {  panic!("Invalid input string! {}", input);  }
        }
    } 
    Ok((row.0,  col.0))
}

fn main() {
    let filename = "input.txt";
    if let Ok(vec) = file_reader::read_to_vec(filename) {
        for line in vec {
            if let Ok((row, seat)) = parse_row_and_seat(&line) {
                println!("row={}; col={}", row, seat);                
            }
        }
    }
}
