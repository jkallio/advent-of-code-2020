use std::fs::File;
use std::io::{ BufRead, BufReader };
use regex::Regex;
mod tile;
use tile::{ Tile, XY };

fn parse_input_file(path: &str) -> Vec<Tile> {

    let file = File::open(&path).unwrap();
    let br = BufReader::new(file);
    
    let mut tiles:Vec<Tile> = vec![];
    let mut tile = Tile::new();

    let mut y = 0;
    for line in br.lines() {
        let line = line.unwrap();
        
        if !line.is_empty() {
            // Prepare for new Tile
            if line.contains("Tile") {
                y = 0;
                if tile.id > 0 {
                    tiles.push(tile);
                }
                tile = Tile::new();
                let re = Regex::new(r"[0-9]+").unwrap();
                if let Some(m) = re.find(&line) {
                    tile.id = *&line[m.start()..m.end()].parse::<i32>().unwrap();
                }
                else {
                    panic!("Invalid Tile title {}", line);
                }
            }
            else {
                for (x, c) in line.chars().enumerate() {
                    let xy = XY { x: x as i32, y: y };
                    tile.pixels.insert(xy, c == '#');
                }
                y += 1;
            }
        }
    }
    tiles.push(tile);

    tiles
}

fn main() {
    let input = "test_input.txt";

    let tiles = parse_input_file(input);
    println!("Total tile count = {}", &tiles.len());
    for mut tile in tiles {
        tile.update_borders();
        tile.print();
    }
}
