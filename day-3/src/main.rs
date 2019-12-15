use std::io;

use std::cmp::Ordering;

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = self.x.abs() + self.y.abs();
        let d2 = other.x.abs() + other.y.abs();

        if d1 > d2 {
            Ordering::Greater
        } else if d1 < d2 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let wire1 = read_wire();
    let wire2 = read_wire();

    let mut distance = 0;
    for p1 in wire1.keys() {
        if let Some(d2) = wire2.get(p1) {
            let step = wire1.get(p1).unwrap() + d2;
            if step < distance || distance == 0 {
                distance = step;
            }
        }
    }
    println!("{:?}", distance);
}

fn read_wire() -> HashMap<Point, usize> {
    // build a set of all points the wire is visiting
    // input format: <dir><num>,<dir><num>,... where <dir> in [R,L,D,U]
    let mut wire_directions = String::new();
    io::stdin().read_line(&mut wire_directions).unwrap();

    let mut hs = HashMap::new();

    // current coordinates
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut distance: usize = 0;

    for instr in wire_directions.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        // let dir = instr.chars().next().unwrap();
        let mut char_vec: Vec<char> = instr.chars().collect();
        let dir = char_vec[0];
        char_vec.remove(0);
        let _mag: String = char_vec.into_iter().collect();
        let mag: u32 = _mag.parse::<u32>().unwrap();

        match dir {
            'R' => {
                for _ in 0..mag {
                    let p = Point{x: x, y: y};
                    x += 1;
                    hs.insert(p, distance);
                    distance += 1;
                }
            },
            'U' => {
                for _ in 0..mag {
                    let p = Point{x: x, y: y};
                    y += 1;
                    hs.insert(p, distance);
                    distance += 1;
                }
            },
            'D' => {
                for _ in 0..mag {
                    let p = Point{x: x, y: y};
                    y -= 1;
                    hs.insert(p, distance);
                    distance += 1;
                }
            },
            'L' => {
                for _ in 0..mag {
                    let p = Point{x: x, y: y};
                    x -= 1;
                    hs.insert(p, distance);
                    distance += 1;
                }
            },
            _ => {
                panic!("Unknown direction: {}", dir);
            },
        }
    }

    // we don't care about the origin for calculations
    hs.remove(&Point{x: 0, y: 0});
    hs
}
