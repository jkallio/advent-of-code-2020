enum Seat {
    INVALID,
    EMPTY,
    OCCUPIED,
    FLOOR,
}

#[derive(Debug)]
enum Direction {
    INVALID,
    NORTH,
    SOUTH,
    EAST,
    WEST,
    NE,
    SE,
    NW,
    SW,
}

// Set `true` for Part2
const PART2: bool = true;

fn get_direction(orig: (i32, i32), pos: (i32, i32)) -> Direction {
    let x = pos.0 - orig.0;
    let y = pos.1 - orig.1;
    match (x, y) {
        (0, 1) => Direction::NORTH,
        (1, 1) => Direction::NE,
        (1, 0) => Direction::EAST,
        (1, -1) => Direction::SE,
        (0, -1) => Direction::SOUTH,
        (-1, -1) => Direction::SW,
        (-1, 0) => Direction::WEST,
        (-1, 1) => Direction::NW,
        _ => Direction::INVALID,
    }
}

fn get_seat_type(table: &[Vec<char>], pos: (i32, i32)) -> Seat {
    let mut seat_type = Seat::INVALID;
    if check_bounds(table, pos) {
        let c = table[pos.0 as usize][pos.1 as usize];
        match c {
            'L' => seat_type = Seat::EMPTY,
            '#' => seat_type = Seat::OCCUPIED,
            '.' => seat_type = Seat::FLOOR,
            _ => seat_type = Seat::INVALID,
        }
    }
    seat_type
}

fn get_seat_type_with_raycast(table: &[Vec<char>], pos: (i32, i32), dir: Direction) -> Seat {
    let mut row = pos.0;
    let mut col = pos.1;
    loop {
        match dir {
            Direction::NORTH => {
                row -= 1;
            }
            Direction::SOUTH => {
                row += 1;
            }
            Direction::EAST => {
                col += 1;
            }
            Direction::WEST => {
                col -= 1;
            }
            Direction::NE => {
                row -= 1;
                col += 1;
            }
            Direction::SE => {
                row += 1;
                col += 1;
            }
            Direction::NW => {
                row -= 1;
                col -= 1;
            }
            Direction::SW => {
                row += 1;
                col -= 1;
            }
            _ => panic!("Invalid direction!"),
        }

        if !check_bounds(&table, (row, col)) {
            return Seat::INVALID;
        } else {
            let seat_type = get_seat_type(&table, (row, col));
            match seat_type {
                Seat::FLOOR => {
                    if !PART2 {
                        // In part2: Raycast ignores the FLOOR and tries to find first seat that hits the raycast
                        return seat_type;
                    }
                }
                _ => {
                    return seat_type;
                }
            }
        }
    }
}

fn check_bounds(table: &[Vec<char>], pos: (i32, i32)) -> bool {
    pos.0 >= 0
        && pos.0 < table.len() as i32
        && pos.1 >= 0
        && pos.1 < table[pos.0 as usize].len() as i32
}

fn count_neighbors(table: &[Vec<char>], pos: (i32, i32)) -> i32 {
    let mut neigh_count = 0;
    for row in (pos.0 - 1)..=(pos.0 + 1) {
        for col in (pos.1 - 1)..=(pos.1 + 1) {
            if row != pos.0 || col != pos.1 {
                let dir = get_direction(pos, (row, col));
                if let Seat::OCCUPIED = get_seat_type_with_raycast(&table, pos, dir) {
                    neigh_count += 1;
                }
            }
        }
    }
    neigh_count
}

fn main() {
    let input_file = "input.txt";
    if let Ok(mut table) = file_utils::read_to_char_table(input_file) {
        let mut occupied_count: i32 = 0;
        loop {
            let mut new_table = table.clone();
            let mut arrangement_stabilized = true;
            for row in 0..table.len() {
                for col in 0..table[row].len() {
                    let seat_type = get_seat_type(&table, (row as i32, col as i32));
                    let neighbors = count_neighbors(&table, (row as i32, col as i32));
                    match seat_type {
                        Seat::EMPTY => {
                            if neighbors == 0 {
                                new_table[row][col] = '#';
                                arrangement_stabilized = false;
                            }
                        }
                        Seat::OCCUPIED => {
                            let mut tolerance = 4;
                            if PART2 {
                                tolerance = 5;
                            }
                            if neighbors >= tolerance {
                                new_table[row][col] = 'L';
                                arrangement_stabilized = false;
                            }
                        }
                        _ => {}
                    }
                }
            }
            table = new_table.clone();
            if arrangement_stabilized {
                break;
            }

            // Count all occupied seats in the table
            occupied_count = 0;
            for row in 0..table.len() {
                for col in 0..table[row].len() {
                    if let Seat::OCCUPIED = get_seat_type(&table, (row as i32, col as i32)) {
                        occupied_count += 1;
                    }
                }
            }
        }
        println!("Num of occupied = {}", occupied_count);
    }
}
