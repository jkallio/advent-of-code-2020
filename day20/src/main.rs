use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod tile;
use tile::{Tile, XY};

fn parse_input_file(path: &str) -> Vec<Tile> {
    let file = File::open(&path).unwrap();
    let br = BufReader::new(file);

    let mut tiles: Vec<Tile> = vec![];
    let mut tile = Tile::new();

    let mut y = 0;
    for line in br.lines() {
        let line = line.unwrap();

        if !line.is_empty() {
            if line.contains("Tile") {
                y = 0;
                if tile.id > 0 {
                    tile.update_borders();
                    tiles.push(tile);
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
    tiles.push(tile);

    tiles
}

fn find_neighbors(tile: &mut Tile, tiles: &[Tile]) {
    for tile_b in tiles.iter() {
        if tile.id != tile_b.id {
            for borders_a in &tile.borders {
                for borders_b in &tile_b.borders {
                    if borders_a.1 .0 == borders_b.1 .0 || borders_a.1 .0 == borders_b.1 .1 {
                        //println!("{} Match {} => {:?} = {:?}", tile.id, tile_b.id, borders_a.1, borders_b.1);
                        tile.neighbors.push(tile_b.id);
                    }
                }
            }
        }
    }
}

fn main() {
    let input = "input.txt";

    let tiles = parse_input_file(input);

    let mut result: i64 = 1;
    for it in &tiles {
        let mut tile = it.clone();
        find_neighbors(&mut tile, &tiles);

        if tile.neighbors.len() == 2 {
            println!("{} is corner", tile.id);
            result *= tile.id as i64;
        }
    }
    println!("Result = {}", result);
}
