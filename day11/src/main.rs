type CharMap = Vec<Vec<char>>;
type Pos = (i32, i32);

enum Seat {
    INVALID,
    EMPTY,
    OCCUPIED,
    FLOOR
}

fn get_seat_type(table:&CharMap, pos:Pos) -> Seat {
    let mut seat_type = Seat::INVALID;
    let row = pos.0;
    let col = pos.1;
    if row >= 0 && row < table.len() as i32 && col >= 0 && col < table[row as usize].len() as i32 {
        let c = table[row as usize][col as usize];
        match c {
            'L' => { seat_type = Seat::EMPTY }
            '#' => { seat_type = Seat::OCCUPIED }
            '.' => { seat_type = Seat::FLOOR }
            _ => { seat_type = Seat::INVALID }
        }
    }
    seat_type
}

fn count_neighbors(table:&CharMap, pos:Pos) -> i32 {
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

fn count_occupied(table:&CharMap) -> i32 {
    let mut occupied_count:i32 = 0;
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
                            if neighbors >= 4 {
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