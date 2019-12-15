use std::io;

use std::cmp::Ordering;

use std::collections::HashSet;

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

    let common = wire1.intersection(&wire2).collect::<Vec<&Point>>();
    // let p: Point = common.min();

    let p = common.iter().min_by(|p1, p2| p1.cmp(p2)).unwrap();
    println!("{}", p.x.abs()+p.y.abs());
    // println!("{:?}", common);
}

fn read_wire() -> HashSet<Point> {
    // build a set of all points the wire is visiting
    // input format: <dir><num>,<dir><num>,... where <dir> in [R,L,D,U]
    let mut wire_directions = String::new();
    io::stdin().read_line(&mut wire_directions).unwrap();

    let mut hs = HashSet::new();

    // current coordinates
    let mut x: i32 = 0;
    let mut y: i32 = 0;

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
                    hs.insert(p);
                }
            },
            'U' => {
                for _ in 0..mag {
                    let p = Point{x: x, y: y};
                    y += 1;
                    hs.insert(p);
                }
            },
            'D' => {
                for _ in 0..mag {
                    let p = Point{x: x, y: y};
                    y -= 1;
                    hs.insert(p);
                }
            },
            'L' => {
                for _ in 0..mag {
                    let p = Point{x: x, y: y};
                    x -= 1;
                    hs.insert(p);
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
