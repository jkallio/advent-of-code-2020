use std::collections::HashMap;

static SIZE: XY = XY { x: 10, y: 10 };

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Side {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct Tile {
    pub id: i32,
    pub pixels: HashMap<XY, bool>,
    pub borders: HashMap<Side, (i32, i32)>,
    pub neighbors: HashMap<Side, i32>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            id: -1,
            pixels: HashMap::<XY, bool>::new(),
            borders: HashMap::<Side, (i32, i32)>::new(),
            neighbors: HashMap::<Side, i32>::new(),
        }
    }

    pub fn flip_x(&mut self) {
        let orig = self.pixels.clone();
        for y in 0..SIZE.y {
            for x in 0..SIZE.x {
                let flipped_pos = XY {
                    x: SIZE.x - x - 1,
                    y,
                };
                self.pixels
                    .insert(flipped_pos, *orig.get(&XY { x, y }).unwrap());
            }
        }
        self.update_borders();
    }

    pub fn flip_y(&mut self) {
        let orig = self.pixels.clone();
        for y in 0..SIZE.y {
            for x in 0..SIZE.x {
                let flipped_pos = XY {
                    x,
                    y: SIZE.y - y - 1,
                };
                self.pixels
                    .insert(flipped_pos, *orig.get(&XY { x, y }).unwrap());
            }
        }
        self.update_borders();
    }

    pub fn rotate(&mut self) {
        let orig = self.pixels.clone();
        for y in 0..SIZE.y {
            for x in 0..SIZE.x {
                let rotated_pos = XY {
                    x: SIZE.x - y - 1,
                    y: x,
                };
                self.pixels
                    .insert(rotated_pos, *orig.get(&XY { x, y }).unwrap());
            }
        }
        self.update_borders();
    }

    pub fn update_borders(&mut self) {
        let mut top = (0, 0);
        let mut bottom = (0, 0);
        let mut left = (0, 0);
        let mut right = (0, 0);
        for x in 0..SIZE.x {
            if *self.pixels.get(&XY { x, y: 0 }).unwrap() {
                top.0 += 1 << x;
                top.1 += 1 << (SIZE.x - 1 - x);
            }
            if *self.pixels.get(&XY { x, y: SIZE.y - 1 }).unwrap() {
                bottom.0 += 1 << x;
                bottom.1 += 1 << (SIZE.x - 1 - x);
            }
        }
        for y in 0..SIZE.y {
            if *self.pixels.get(&XY { x: 0, y }).unwrap() {
                left.0 += 1 << y;
                left.1 += 1 << (SIZE.y - 1 - y);
            }
            if *self.pixels.get(&XY { x: SIZE.x - 1, y }).unwrap() {
                right.0 += 1 << y;
                right.1 += 1 << (SIZE.y - 1 - y);
            }
        }
        self.borders.insert(Side::TOP, top);
        self.borders.insert(Side::BOTTOM, bottom);
        self.borders.insert(Side::LEFT, left);
        self.borders.insert(Side::RIGHT, right);
    }

    pub fn print_pixels(&self) {
        for y in 0..SIZE.y {
            for x in 0..SIZE.x {
                let mut c = '.';
                if *self.pixels.get(&XY { x, y }).unwrap() {
                    c = '#';
                }
                print!("{}", c);
            }
            println!();
        }
    }

    pub fn print_borders(&self) {
        println!(
            "Tile {}: Top=({:?}); Bottom=({:?}); Left=({:?}); Right=({:?})",
            self.id,
            self.borders.get(&Side::TOP).unwrap(),
            self.borders.get(&Side::BOTTOM).unwrap(),
            self.borders.get(&Side::LEFT).unwrap(),
            self.borders.get(&Side::RIGHT).unwrap()
        );
    }
}
