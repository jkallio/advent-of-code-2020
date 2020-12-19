use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Hash, PartialEq, Eq, Clone)]
struct XYZW {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

struct Bounds {
    min: XYZW,
    max: XYZW,
}

type CubeMap = HashMap<XYZW, bool>;

// Parse input file into a xyzw map where each position represents the
// state of a single energy source (´true´ if energy soruce is active)
fn parse_input_file(input: &str) -> (CubeMap, Bounds) {
    let file = File::open(input).unwrap();
    let br = BufReader::new(file);

    let mut cubes = HashMap::<XYZW, bool>::new();
    let mut pos = XYZW { x: 0, y: 0, z: 0, w:0 };
    let mut bounds = Bounds {
        min: XYZW { x: 0, y: 0, z: 0, w:0 },
        max: XYZW { x: 0, y: 0, z: 0, w:0 },
    };
    for line in br.lines() {
        if let Ok(line) = line {
            for c in line.chars() {
                match c {
                    '.' => {
                        cubes.insert(pos.clone(), false);
                    }
                    '#' => {
                        cubes.insert(pos.clone(), true);
                    }
                    _ => {
                        panic!("Invalid input file");
                    }
                }
                pos.x = (pos.x + 1) % line.len() as i32;
                bounds.max.x = max(bounds.max.x, pos.x + 1);
            }
        } else {
            panic!("Invalid line in input file");
        }
        pos.y += 1;
        bounds.max.y = max(bounds.max.y, pos.y);
    }
    (cubes, bounds)
}

// Returns true if energy cell is active at given position
fn is_active(cubes: &CubeMap, pos: &XYZW) -> bool {
    let mut active = false;
    if let Some(value) = cubes.get(&pos) {
        active = *value;
    }
    active
}

// Counts all active energy cells within given bounds
fn count_active_cubes(cubes: &CubeMap, bounds: &Bounds) -> i32 {
    let mut active_count = 0;
    for w in bounds.min.w..=bounds.max.w {
        for z in bounds.min.z..=bounds.max.z {
            for y in bounds.min.y..=bounds.max.y {
                for x in bounds.min.x..=bounds.max.x {
                    if is_active(&cubes, &XYZW { x, y, z, w}) {
                        active_count += 1;
                    }
                }
            }
        }
    }
    active_count
}

// Toggles the state of energy cells based on neighbor's activity state
fn execute_bootup_cycle(cubes: &mut CubeMap, bounds: &Bounds) {
    let mut vec = vec![];
    for own_w in bounds.min.w..=bounds.max.w {
        for own_z in bounds.min.z..=bounds.max.z {
            for own_y in bounds.min.y..=bounds.max.y {
                for own_x in bounds.min.x..=bounds.max.x {
                    let own_pos = XYZW {
                        x: own_x,
                        y: own_y,
                        z: own_z,
                        w: own_w,
                    };
                    let mut active_neigbors = count_active_cubes(
                        &cubes,
                        &Bounds {
                            min: XYZW {
                                x: own_x - 1,
                                y: own_y - 1,
                                z: own_z - 1,
                                w: own_w - 1,
                            },
                            max: XYZW {
                                x: own_x + 1,
                                y: own_y + 1,
                                z: own_z + 1,
                                w: own_w + 1,
                            },
                        },
                    );

                    // If a cube is active and exactly 2 or 3 of its neighbors
                    // are also active, the cube remains active. Otherwise,
                    // the cube becomes inactive.
                    if is_active(&cubes, &own_pos) {
                        active_neigbors -= 1;
                        if active_neigbors != 2 && active_neigbors != 3 {
                            vec.push(own_pos);
                        }
                    }
                    // If a cube is inactive but exactly 3 of its neighbors are
                    // active, the cube becomes active. Otherwise, the cube remains
                    // inactive.
                    else if active_neigbors == 3 {
                        vec.push(own_pos);
                    }
                }
            }
        }
    }

    // Toggle state of energy cells in collected positions
    for p in vec {
        let new_state = !is_active(&cubes, &p);
        cubes.insert(p.clone(), new_state);
    }
}

fn main() {
    let input = "input.txt";
    let (mut cubes, mut bounds) = parse_input_file(input);

    for i in 1..=6 {
        // Increase the bounds to each dimension
        bounds.min.x -= 1;
        bounds.min.y -= 1;
        bounds.min.z -= 1;
        bounds.min.w -= 1;
        bounds.max.x += 1;
        bounds.max.y += 1;
        bounds.max.z += 1;
        bounds.max.w += 1;

        // Execute bootup cycle
        execute_bootup_cycle(&mut cubes, &bounds);

        // Count active energy sources
        let active = count_active_cubes(&cubes, &bounds);
        println!("Cycle #{}: Active energy soruces {}", i, active);
    }
}
