fn parse_line(line: String) -> (char, i32) {
    if let Some(a) = &line[..1].chars().next() {
        if let Ok(val) = &line[1..].parse() {
            return (*a, *val);
        }
    }
    panic!("Failed to parse line");
}

fn rotate(point: (i32, i32), deg: i32) -> (i32, i32) {
    let mut d = deg;
    while d < 0 {
        d += 360;
    }

    match d % 360 {
        0 => point,

        90 => (point.1, -point.0),

        180 => (-point.0, -point.1),

        270 => (-point.1, point.0),
        _ => {
            panic!("Invalid direction");
        }
    }
}

fn move_towards(dir: (i32, i32), value: i32) -> (i32, i32) {
    let mut result = (0, 0);
    for _ in 0..value {
        result.0 += dir.0;
        result.1 += dir.1;
    }
    result
}

fn get_coordinates(dir: char, value: i32) -> (i32, i32) {
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
        let mut waypoint = (10, 1);

        for line in input {
            let cmd = parse_line(line);
            match cmd.0 {
                'L' => {
                    waypoint = rotate(waypoint, -cmd.1);
                }

                'R' => {
                    waypoint = rotate(waypoint, cmd.1);
                }

                'F' => {
                    let result = move_towards(waypoint, cmd.1);
                    pos.0 += result.0;
                    pos.1 += result.1;
                }

                _ => {
                    let coords = get_coordinates(cmd.0, cmd.1);
                    waypoint.0 += coords.0;
                    waypoint.1 += coords.1;
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
