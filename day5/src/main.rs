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

// Find empty seats (and print the seatmap at the same time)
//  + seat_map: 2-dimensional boolean vector where 'true' indicates occupied seat
fn find_empty_seats(seat_map: &Vec<Vec<bool>>) -> Vec<u16> {
    let mut empty_seats = Vec::new();
    for (i, row) in seat_map.iter().enumerate() {
        let mut id = 0;
        for (j, seat) in row.iter().enumerate() {
            if !seat { 
                print!("X ");
                id = calculate_seat_id(i as i32, j as i32);
                empty_seats.push(id as u16);
            }
            else { print!(". "); }
        }
        println!(" - {} ({})", i, id);
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

        // Find empty seats
        let empty_seats = find_empty_seats(&seats);

        // My seat should be the last free seat. Note, that the plane does not actually have the seats in the front and the back. 
        // It was given that both id-1 and id+1 seats should be occupied. Find such seat from the empty_seats vector
        let my_id = &empty_seats.windows(3).find(|i| i[1] != i[0] + 1 && i[1] != i[2]-1).unwrap()[1];
        println!("Your seat ID is {}", my_id);
    }
}
