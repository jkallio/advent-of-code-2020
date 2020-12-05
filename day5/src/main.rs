mod jk;
use jk::file_reader;

// Parse Row & Seat using binary space partitioning
//  + input:    Input string to be parsed
//  -> Returns (row, col) if valid input given; Error otherwise
fn parse_row_and_seat(input: &str) -> Result<(i32, i32), std::io::Error> {
    let mut row = (0, 127);
    let mut col = (0, 7);
    
    for a in input.chars() {
        match a {
            'F' => row.1 = row.1 - (row.1 - row.0 + 1) / 2,
            'B' => row.0 = row.0 + (row.1 - row.0 + 1) / 2,
            'L' => col.1 = col.1 - (col.1 - col.0 + 1) / 2,
            'R' => col.0 = col.0 + (col.1 - col.0 + 1) / 2,
            _ => {  panic!("Invalid input string! {}", input);  }
        }
    } 
    assert_eq!(row.0 == row.1 && col.0 == col.1, true);
    Ok((row.0,  col.0))
}

// Calucates seat id for given row & col
fn calculate_seat_id(row:i32, col:i32) -> i32 {
    return row * 8 + col;
}

// Find empty seats
fn find_empty_seats(seat_map: &Vec<Vec<bool>>) -> Vec<i32> {
    let mut empty_seats = Vec::new();
    for (i, row) in seat_map.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            if !seat {
                empty_seats.push(calculate_seat_id(i as i32, j as i32));
            }
        }
    }
    return empty_seats;
}

// Main function
fn main() {
    let filename = "input.txt";
    if let Ok(vec) = file_reader::read_to_vec(filename) {

        // Create seat map for the plane
        let mut seats = vec![vec![false;8]; 128];

        // Parse each boarding pass and mark according seat as occupied
        for line in vec {
            if let Ok((row, col)) = parse_row_and_seat(&line) {
                seats[(row as usize)][(col as usize)] = true;
            }
        }
        let empty_seats = find_empty_seats(&seats);
        for id in empty_seats {
            println!("{}", id);
        }
    }
}

// Print the seat map
//  + seat_map: 2-dimensional boolean vector where 'true' indicates occupied seat
fn print_seat_map(seat_map: &Vec<Vec<bool>>) {

    for (i, row) in seat_map.iter().enumerate() {
        for seat in row {
            if !seat { print!("X "); }
            else { print!("."); }
        }
        println!(" - {}", i);
    }
}
