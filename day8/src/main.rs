mod jk;
use jk::file_reader;

fn parse_line(line:&str) -> Result<(&str, i32), &'static str> {
    let mut parts = line.split_whitespace();
    if let Some(cmd) = parts.next() {
        if let Some(value) = parts.next() {
            if let Ok(parsed_value) = value.parse::<i32>() {
                return Ok((cmd, parsed_value));
            }
        }
    }
    panic!("Failed to parse value from line {}", line);
}

fn main() {
    let filename = "input.txt";
    if let Ok(vec) = file_reader::read_to_vec(filename) {
        let mut pos:i32 = 0;
        let mut accumulator:i32 = 0;
        let mut history = Vec::<i32>::new();
        loop {
            if history.contains(&pos) {
                println!("Infinite loop detected!");
                break;
            }
            history.push(pos);
            let line = &vec[pos as usize];
            history.push(pos);
            if let Ok(cmd) = parse_line(&line) {
                match cmd.0 {
                    "acc" => { accumulator += cmd.1; pos += 1; }
                    "jmp" => { pos += cmd.1; }
                    "nop" => { pos += 1; }
                    _ => { panic!("Invalid command: {}", cmd.0); }
                }
            }
        }
        println!("Accumulator value {}", accumulator);
    }
}
