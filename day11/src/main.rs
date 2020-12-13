enum Seat {
    INVALID,
    EMPTY,
    OCCUPIED,
    FLOOR,
}

// Part1: Tolerance is 4. Part2: Tolerance is 5.
const TOLERANCE: i32 = 4;

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
            let seat_type = get_seat_type(&table, (row, col));
            if let Seat::OCCUPIED = seat_type {
                if row != pos.0 || col != pos.1 {
                    neigh_count += 1;
                }
            }
        }
    }
    neigh_count
}

fn count_occupied(table: &[Vec<char>]) -> i32 {
    let mut occupied_count: i32 = 0;
    for row in 0..table.len() {
        for col in 0..table[row].len() {
            let seat_type = get_seat_type(&table, (row as i32, col as i32));
            if let Seat::OCCUPIED = seat_type {
                occupied_count += 1;
            }
        }
    }
    occupied_count
}

fn main() {
    let input_file = "input.txt";
    if let Ok(mut table) = file_utils::read_to_char_table(input_file) {
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
                            if neighbors >= TOLERANCE {
                                new_table[row][col] = 'L';
                                arrangement_stabilized = false;
                            }
                        }
                        _ => {}
                    }
                }
            }
            table = new_table.clone();
            let occupied_count = count_occupied(&table);
            if arrangement_stabilized {
                break;
            }
            println!("Num of occupied = {}", occupied_count);
        }
    }
}
