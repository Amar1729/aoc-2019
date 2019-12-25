use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug)]
struct SimplePoint {
    x: i8,
    y: i8,
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

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut asteroids: Vec<(i8, i8)> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        for (j, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                asteroids.push((j as i8, i as i8));
            }
        }
    }

    // calculate count for each asteroid
    let result = asteroids.iter().map(|a| calc(*a, &asteroids)).max().unwrap();
    println!("{}", result);
}

fn calc(point: (i8, i8), asteroids: &[(i8, i8)]) -> usize {

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
