use std::collections::HashMap;
use std::rc::Rc;

static size:XY = XY { x:10, y:10 };

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Side {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct XY {
    pub x: i32,
    pub y: i32
}

pub struct Tile {
    pub id: i32,
    pub pixels: HashMap<XY, bool>,
    pub borders: HashMap<Side, (i32, i32)>,
    pub neighbors: Vec<Rc<Tile>>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            id: -1,
            pixels: HashMap::<XY, bool>::new(),
            borders: HashMap::<Side, (i32, i32)>::new(),
            neighbors: vec![],
        }
    }

    pub fn flip(&mut self) {
        let orig = self.pixels.clone();
        for y in 0..size.y {
            for x in 0..size.x {
                let flipped_pos = XY { x: size.x - x, y };
                self.pixels.insert(flipped_pos, *orig.get(&XY{x,y}).unwrap());
            }
        }
        self.update_borders();
    }
    
    pub fn rotate(&mut self) {
        let orig = self.pixels.clone();
        for y in 0..size.y {
            for x in 0..size.x {
                let rotated_pos = XY { x: size.x - y, y: x };
                self.pixels.insert(rotated_pos, *orig.get(&XY{x,y}).unwrap());
            }
        }
        self.update_borders();
    }

    pub fn update_borders(&mut self) {
        for x in 0..size.x {
            if *self.pixels.get(&XY{x, y:0}).unwrap() {
                let mut top = self.borders.entry(Side::TOP).or_insert((0,0));
                top.1 += 1 << x;
                top.0 += 1 << (size.x-1 - x);
            }
            if *self.pixels.get(&XY{x, y:size.y-1}).unwrap() {
                let mut bottom = self.borders.entry(Side::BOTTOM).or_insert((0,0));
                bottom.1 += 1 << x;
                bottom.0 += 1 << (size.x-1 - x);
            }
        }
        for y in 0..size.y {
            if *self.pixels.get(&XY{x:0, y}).unwrap() {
                let mut left = self.borders.entry(Side::LEFT).or_insert((0,0));
                left.1 += 1 << y;
                left.0 += 1 << (size.y-1 - y);
            }
            if *self.pixels.get(&XY{x:size.x-1, y}).unwrap() {
                let mut right = self.borders.entry(Side::RIGHT).or_insert((0,0));
                right.1 += 1 << y;
                right.0 += 1 << (size.y-1 - y);
            }    
        }
    }

    pub fn print(&self) {
        println!("Tile {}: Top=({}/{}); Bottom=({}/{}); Left=({}/{}); Right=({}/{})", self.id, 
            self.borders.get(&Side::TOP).unwrap().0,
            self.borders.get(&Side::TOP).unwrap().1,
            self.borders.get(&Side::BOTTOM).unwrap().0,
            self.borders.get(&Side::BOTTOM).unwrap().1,
            self.borders.get(&Side::LEFT).unwrap().0,
            self.borders.get(&Side::LEFT).unwrap().1,
            self.borders.get(&Side::RIGHT).unwrap().0,
            self.borders.get(&Side::RIGHT).unwrap().1);

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

    /*
    pub fn find_neighbors(&mut self, tiles:&[Tile]) {
    }
    */
}

