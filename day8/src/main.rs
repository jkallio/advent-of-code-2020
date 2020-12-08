use std::fs::File;
use std::io::{BufRead, BufReader};

// Read the input file as list of (String, i32) pair
fn parse_input_file(path:&str) -> Result<Vec<(String, i32)>, &'static str> {
    if let Ok(file) = File::open(path) {
        let br = BufReader::new(file);
        let mut code = Vec::<(String, i32)>::new();

        for line in br.lines() {
            if let Ok(l) = line {
                if let Ok(cmd) = parse_line(&l) {
                    code.push((String::from(cmd.0), cmd.1));
                }
            }
        }
        return Ok(code);
    }
    return Err("Failed to read file");
}

// Parse given line as (&str, i32) pair
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

// Run given code instruction by instruction
fn run_code(code: &Vec<(String, i32)>, fix_count:i32) -> Result<i32, &'static str> {
    let mut pos:i32 = 0;
    let mut acc:i32 = 0;
    let mut instruction_counter = 0;
    let mut history = Vec::<i32>::new();
    loop {
        if pos == code.len() as i32 {
            return Ok(acc);
        }
        else if history.contains(&pos) {
            println!("Infinite loop found; Acc={}", acc);
            return Err("Infinite loop");
        }
        else {
            history.push(pos);
            if pos >= 0 && pos < code.len() as i32 {
                let cmd = &code[pos as usize];
                if cmd.0.contains("acc") {
                    acc += cmd.1;
                    pos += 1;
                }
                else {
                    // Exactly one 'jmp' or 'nop' instruction is corrupted in the given code vector
                    // So fix (swap) the nth instruction and try if the code runs 'til the end
                    instruction_counter += 1;
                    if cmd.0.contains("jmp") ^ (instruction_counter == fix_count) {
                        pos += cmd.1;
                    }
                    else if cmd.0.contains("nop") ^ (instruction_counter == fix_count) {
                        pos += 1;
                    }
                    else {
                        panic!("Invalid command {}", cmd.0);
                    }
                }
            }
        }
    }
}

// Main function
fn main() {
    let filename = "input.txt";
    if let Ok(code) = parse_input_file(&filename) {
        let mut fix_counter = 1;
        
        // Try to run the code until it runs 'til the end
        // Increment fix_counter for iteration.
        loop {
            if let Ok(acc) = run_code(&code, fix_counter) {
                println!("Found the working code with corrupt_pos={}; Acc={}", fix_counter, acc);
                break;
            }
            else {
                println!("Code did not run with {}", fix_counter);
                fix_counter += 1;
            }
      }
    }
}
