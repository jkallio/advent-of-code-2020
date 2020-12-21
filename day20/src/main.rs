use std::fs::File;
use std::io::{ BufRead, BufReader };
use regex::Regex;
use std::collections::HashMap;

type TileMap = Vec<Tile>;

#[derive(Hash, PartialEq, Eq, Clone)]
struct XY {
    x: i32,
    y: i32
}

struct Tile {
    id: i32,
    pixels: HashMap<XY, bool>,
    borders: Vec<i32>,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            id: -1,
            pixels: HashMap::<XY, bool>::new(),
            borders: vec![],
        }
    }
}

fn parse_input_file(path: &str) -> TileMap {

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

fn calculate_border_identifiers(tiles: &mut TileMap) {
    for (i, tile) in tiles.iter().enumerate() {
        
    }
}


fn main() {
    let input = "test_input.txt";

    let mut tiles = parse_input_file(input);
    calculate_border_identifiers(&mut tiles);

    for tile in tiles {
        println!("{}", tile.id);

        for y in 0..10 {
            for x in 0..10 {
                let mut c = '.';
                if *tile.pixels.get(&XY {x,y}).unwrap() {
                    c = '#';
                }
                print!("{}", c);
            }
            println!("");
        }
        println!("");
    }


}
