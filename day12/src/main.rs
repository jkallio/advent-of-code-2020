fn parse_line(line: String) -> (char, i32) {
    if let Some(a) = &line[..1].chars().next() {
        if let Ok(val) = &line[1..].parse() {
            return (*a, *val);
        }
    }
    panic!("Failed to parse line");
}

fn deg_to_dir(deg: i32) -> char {
    match deg % 360 {
        0 => 'N',
        90 => 'E',
        180 => 'S',
        270 => 'W',
        _ => panic!("Invalid degree"),
    }
}

fn dir_to_deg(dir: char) -> i32 {
    match dir {
        'N' => 0,
        'S' => 180,
        'E' => 90,
        'W' => 270,
        _ => panic!("Invalid direction"),
    }
}

fn turn(deg: i32, cur_dir: char) -> char {
    let mut new_deg = deg + dir_to_deg(cur_dir);
    while new_deg < 0 {
        new_deg += 360
    }
    deg_to_dir(new_deg)
}

fn move_direction(dir: char, value: i32) -> (i32, i32) {
    let mut result = (0, 0);
    match dir {
        'N' => {
            result.1 += value;
        }

        'S' => {
            result.1 -= value;
        }

        'E' => {
            result.0 += value;
        }

        'W' => {
            result.0 -= value;
        }

        _ => {
            panic!("Invalid direction");
        }
    }
    result
}

fn main() {
    if let Ok(input) = file_utils::read_to_string_vec("input.txt") {
        let mut pos = (0, 0);
        let mut facing = 'E';
        for line in input {
            let cmd = parse_line(line);
            match cmd.0 {
                'L' => {
                    facing = turn(-cmd.1, facing);
                }

                'R' => {
                    facing = turn(cmd.1, facing);
                }

                'F' => {
                    let result = move_direction(facing, cmd.1);
                    pos.0 += result.0;
                    pos.1 += result.1;
                }

                _ => {
                    let result = move_direction(cmd.0, cmd.1);
                    pos.0 += result.0;
                    pos.1 += result.1;
                }
            }
        }
        println!(
            "{} {} => distance = {}",
            pos.0,
            pos.1,
            (pos.0.abs() + pos.1.abs())
        );
    }
}
