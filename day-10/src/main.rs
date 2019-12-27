use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::f32;

use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Hash, Debug)]
struct SimplePoint {
    x: i8,
    y: i8,
}

#[derive(Debug)]
struct FullPoint {
    sp: SimplePoint,
    p: (i8, i8),

    rank: usize,
    deg: f32,
}

impl SimplePoint {
    fn new(x: i8, y: i8) -> SimplePoint {
        let mut _x = 0;
        let mut _y = 0;

        if x == 0 && y == 0 {
            SimplePoint { x: x, y: y }
        } else if x == 0 {
            // special case: point is on x axis
            if y > 0 {
                _y = 1;
            } else if y < 0 {
                _y = -1;
            }

            SimplePoint { x: x, y: _y }
        } else if y == 0 {
            // special case: point is on y axis
            if x > 0 {
                _x = 1;
            } else if y < 0 {
                _x = -1;
            }

            SimplePoint { x: _x, y: y }
        } else {
            // for a nontrivial point, simplify it (treating it as fraction)

            // calculate gcd(x, y)
            fn gcd(x: i8, y: i8) -> i8 {
                // will return positive number
                for i in (2..std::cmp::min(i8::abs(x), i8::abs(y))+1).rev() {
                    if x%i == 0 && y%i == 0 {
                        return i as i8;
                    }
                }

                return 1;
            }

            let div = gcd(x, y);
            _x = x / div;
            _y = y / div;

            SimplePoint {
                x: _x as i8,
                y: _y as i8,
            }
        }
    }
}

impl FullPoint {
    fn new(x: i8, y: i8) -> FullPoint {
        let mut deg = if x == 0 && y == 0 {
            panic!("don't address origin")
        } else if y == 0 {
            if x > 0 {
                0.
            } else {
                180.
            }
        } else if x == 0 {
            if y > 0 {
                90.
            } else {
                270.
            }
        } else {
            let _deg = (y as f32/x as f32).atan() * 180. / f32::consts::PI;
            if x < 0 && y < 0 {
                _deg + 180.
            } else {
                _deg
            }
        } + 90.;

        if deg as u32 >= 360 {
            deg -= 360.;
        }

        FullPoint {
            sp: SimplePoint::new(x, y),
            p: (x, y),
            rank: 0,
            deg: deg,
        }
    }

    fn increment(&mut self) {
        self.rank += 1;
    }
}

impl Eq for FullPoint { }

impl PartialEq for FullPoint {
    fn eq(&self, other: &Self) -> bool {
        self.sp == other.sp && self.p == other.p
    }
}

impl Ord for FullPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sp == other.sp {
            let dist_self = i8::abs(self.p.0) + i8::abs(self.p.1);
            let dist_other = i8::abs(other.p.0) + i8::abs(other.p.1);
            if dist_self > dist_other {
                Ordering::Greater
            } else if dist_self < dist_other {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        } else {
            self.deg.partial_cmp(&other.deg).unwrap()
        }
    }
}

impl PartialOrd for FullPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut asteroids: Vec<(i8, i8)> = Vec::new();

    let mut point: Option<(i8, i8)> = None;

    for (i, line) in reader.lines().enumerate() {
        for (j, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                asteroids.push((j as i8, i as i8));
            }

            if c == 'X' {
                point = Some((j as i8, i as i8));
                println!("found result: {:?}", point);
            }
        }
    }

    /* p1:
    // calculate count for each asteroid
    let result = asteroids.iter().map(|a| find_count(*a, &asteroids)).max().unwrap();
    println!("{}", result);
    */

    match point {
        None => {
            println!("finding result ...");
            // find the location of the asteroid center
            let result: ((i8, i8), usize) =
                asteroids
                .iter().map(|a| (*a, find_count(*a, &asteroids)))
                .fold(((0, 0), 0), |ast, other| {
                    if other.1 > ast.1 { other } else { ast }
                });

            point = Some(result.0);
        },
        _ => {},
    }

    // assume point is set here
    println!("using result: {:?}", point);
    let point = point.unwrap();

    let mut corrected: Vec<FullPoint> =
        asteroids.iter()
        .map(|p| (p.0 - point.0, p.1 - point.1))
        .filter(|p| !(p.0 == 0 && p.1 == 0))  // can i do FilterMap instead?
        .map(|a| FullPoint::new(a.0, a.1))
        .collect();

    corrected.sort();

    {
        let mut prev = &corrected[0];
        for i in 1..corrected.len() {
            // Eq checks that the SimplePoints of each are the same
            if corrected[i].sp == (*prev).sp {
                corrected[i].increment();
            }
            prev = &corrected[i];
        }
    }

    let mut rank = 0;
    let mut count = 0;

    loop {
        let mut i = 0;
        let mut p: (i8, i8) = (0, 0);
        while i < corrected.len() {
            let curr = &corrected[i];
            p = curr.p.clone();
            if curr.rank <= rank {
                // println!("{} destroy: {:?}", count, curr.p);
                corrected.remove(i);
                count += 1;
            } else {
                // println!("{} skip: {:?}", count, curr.p);

                i += 1;
            }

            if count == 200 {
                break;
            }
        }

        rank += 1;

        if count == 200 {
            let adj = (p.0 as i32 + point.0 as i32, p.1 as i32 + point.1 as i32);
            println!("{:?}", adj);
            let result = 100 * adj.0 + adj.1;
            println!("{:?}", result);

            break;
        }

        // dbg
        if corrected.len() == 0 { panic!("empty queue after: {}", count); }
    }
}

fn find_count(point: (i8, i8), asteroids: &[(i8, i8)]) -> usize {

    let mut slope_set: HashSet<SimplePoint> = HashSet::new();

    // for ease of calculations, adjust the map so given point is the origin of the map (0, 0)
    for adjusted_point in asteroids.iter().map(|p| (p.0 - point.0, p.1 - point.1)) {
        // (don't add ourself)
        if !(adjusted_point.0 == 0 && adjusted_point.1 == 0) {
            slope_set.insert(SimplePoint::new(adjusted_point.0, adjusted_point.1));
        }
    }

    return slope_set.len();
}
