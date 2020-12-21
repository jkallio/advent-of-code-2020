use std::collections::HashMap;

pub type TileMap = Vec<Tile>;

static size:XY = XY { x:10, y:10 };

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct XY {
    pub x: i32,
    pub y: i32
}

pub struct Tile {
    pub id: i32,
    pub pixels: HashMap<XY, bool>,
    pub top: (i32, i32),
    pub bottom: (i32, i32),
    pub left: (i32, i32),
    pub right: (i32, i32),
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            id: -1,
            pixels: HashMap::<XY, bool>::new(),
            top: (0,0),
            bottom: (0,0),
            left: (0,0),
            right: (0,0),
        }
    }

    pub fn flip(&mut self) {
        let orig = self.pixels.clone();
        for y in 0..size.y {
            for x in 0..size.x {
                let pos = XY { x: size.x - x, y };
                self.pixels.insert(pos, *orig.get(&XY{x,y}).unwrap());
            }
        }
    }

    pub fn update_borders(&mut self) {
        // Top & bottom borders
        for x in 0..size.x {
            if *self.pixels.get(&XY{x, y:0}).unwrap() {
                self.top.1 += 1 << x;
                self.top.0 += 1 << (9 - x);
            }
            if *self.pixels.get(&XY{x, y:size.y-1}).unwrap() {
                self.bottom.1 += 1 << x;
                self.bottom.0 += 1 << (9 - x);
            }
        }

        // Left border 
        for y in 0..size.y {
            if *self.pixels.get(&XY{x:0, y}).unwrap() {
                self.left.1 += 1 << y;
                self.left.0 += 1 << (9 - y);
            }
            if *self.pixels.get(&XY{x:size.x-1, y}).unwrap() {
                self.right.1 += 1 << y;
                self.right.0 += 1 << (9 - y);
            }
        }
    }

    pub fn print(&self) {
        println!("Tile {}: Top=({}/{}); Bottom=({}/{}); Left=({}/{}); Right=({}/{})", self.id, 
                self.top.0, self.top.1,
                self.bottom.0, self.bottom.1,
                self.left.0, self.left.1,
                self.right.0, self.right.1);
        for y in 0..size.y {
            for x in 0..size.x {
                let mut c = '.';
                if *self.pixels.get(&XY {x,y}).unwrap() {
                    c = '#';
                }
                print!("{}", c);
            }
            println!("");
        }
    }
}

