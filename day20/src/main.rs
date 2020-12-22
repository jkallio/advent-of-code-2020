use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod tile;
use tile::{Tile, XY, Side};
use std::collections::HashMap;

fn parse_input_file(path: &str) -> HashMap<i32, Tile> {
    let file = File::open(&path).unwrap();
    let br = BufReader::new(file);

    let mut tiles = HashMap::<i32, Tile>::new();
    let mut tile = Tile::new();

    let mut y = 0;
    for line in br.lines() {
        let line = line.unwrap();

        if !line.is_empty() {
            if line.contains("Tile") {
                y = 0;
                if tile.id > 0 {
                    tile.update_borders();
                    tiles.insert(tile.id, tile);
                }
                tile = Tile::new();
                let re = Regex::new(r"[0-9]+").unwrap();
                if let Some(m) = re.find(&line) {
                    tile.id = line[m.start()..m.end()].parse::<i32>().unwrap();
                } else {
                    panic!("Invalid Tile title {}", line);
                }
            } else {
                for (x, c) in line.chars().enumerate() {
                    let xy = XY { x: x as i32, y };
                    tile.pixels.insert(xy, c == '#');
                }
                y += 1;
            }
        }
    }
    tile.update_borders();
    tiles.insert(tile.id, tile);

    tiles
}

fn find_neighbors(tile: &Tile, tiles: &HashMap<i32, Tile>) -> HashMap<Side, i32> {
    let mut neighbors = HashMap::<Side, i32>::new();
    for neighbor in tiles.values() {
        if tile.id != neighbor.id {
            for tile_border in &tile.borders {
                for neigh_border in &neighbor.borders {
                    if tile_border.1 .0 == neigh_border.1 .0 || tile_border.1 .0 == neigh_border.1 .1 {
                        neighbors.insert(tile_border.0.clone(), neighbor.id);
                    }
                }
            }
        }
    }
    neighbors
}

fn main() {
    let input = "test_input.txt";

    let tiles = parse_input_file(input);
    for tile in tiles.values() {
        print!("Neigbors for {} => ", tile.id);
        for n in find_neighbors(&tile, &tiles) {
            print!(" {:?} = {};", n.0, n.1);
        }
        println!();
    }
}
