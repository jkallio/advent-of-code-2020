use std::fs::File;
use std::io::{ BufRead, BufReader };
use regex::Regex;

type TileMap = Vec<Tile>;

struct Tile {
    id: i32,
    lines: Vec<String>
}

impl Tile {
    fn new() -> Tile {
        Tile {
            id: -1,
            lines: vec![]
        }
    }
}

fn parse_input_file(path: &str) -> TileMap {

    let file = File::open(&path).unwrap();
    let br = BufReader::new(file);
    
    let mut tiles:Vec<Tile> = vec![];
    let mut tile = Tile::new();
    for it in br.lines() {
        let line = it.unwrap();
        if !line.is_empty() {
            if line.contains("Tile") {
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
                tile.lines.push(line);
            }
        }
    }
    tiles.push(tile);

    tiles
}

fn main() {
    let input = "test_input.txt";

    let tiles = parse_input_file(input);
    for tile in tiles {
        println!("{}", tile.id);
        for line in tile.lines {
            println!("{}", line);
        }
    }


}
