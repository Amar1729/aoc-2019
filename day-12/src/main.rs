#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

// Moon:
// time step:
// - update velocity by gravity
// - update position by velocity

// gravity:
// v1 and v2 get closer by +- 1

// velocity:
// px + vx, py + vy, pz + vz

// PE = abs(x) + ...
// KE = abs(vx) + ...
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

impl Moon {
    fn new(pos: [i32; 3]) -> Moon {
        Moon {
            pos: pos,
            vel: [0, 0, 0],
        }
    }

    fn clone(&self) -> Moon {
        Moon {
            pos: self.pos,
            vel: self.vel,
        }
    }

    fn update_vel(&mut self, other: &mut Self) {
        for i in 0..3 {
            if self.pos[i] > other.pos[i] {
                self.vel[i] -= 1;
                other.vel[i] += 1;
            } else if self.pos[i] < other.pos[i] {
                self.vel[i] += 1;
                other.vel[i] -= 1;
            }
        }
    }

    fn update_pos(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }

    fn energy(&self) -> i32 {
        let pe: i32 = self.pos.iter().map(|i| i32::abs(*i)).sum();
        let ke: i32 = self.vel.iter().map(|i| i32::abs(*i)).sum();

        pe * ke
    }
}

impl fmt::Debug for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos=<x={:>3}, y={:>3}, z={:>3}>, vel=<x={:>3}, y={:>3}, z={:>3}>",
           self.pos[0],
           self.pos[1],
           self.pos[2],
           self.vel[0],
           self.vel[1],
           self.vel[2],
       )
    }
}

impl FromStr for Moon {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> =
            s.trim_matches(|p| p == '<' || p == '>')
            .split(',')
            .filter(|p| !p.is_empty())
            .map(|p| p.trim_matches(|p| p == ' '))
            .map(|p| p.replace("x=", ""))
            .map(|p| p.replace("y=", ""))
            .map(|p| p.replace("z=", ""))
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        Ok(
            Moon {
                pos: [
                    coords[0],
                    coords[1],
                    coords[2],
                ],
                vel: [0, 0, 0],
            }
        )
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut moons: Vec<Moon> =
        reader
        .lines()
        .map(|l| Moon::from_str(&l.unwrap()).unwrap())
        .collect();

    for i in 0..1000 + 1 {
        if i > 0 {
            time_step(&mut moons);
        }

        /*
        println!("{} steps:", i);
        print_system(&moons);
        println!();
        */

    }

    let energy: i32 = moons.iter().map(|m| m.energy()).sum();
    println!("{:?}", energy);
}

fn time_step(moons: &mut[Moon]) {
    // execute one time step:

    // update velocity
    for i in 0..moons.len() {
        for j in i+1..moons.len() {
            let mut prev = moons[i].clone();
            let mut curr = moons[j].clone();

            prev.update_vel(&mut curr);

            moons[i] = prev;
            moons[j] = curr;
        }
    }

    // update positions
    for m in moons {
        m.update_pos();
    }
}

fn print_system(moons: &[Moon]) {
    for m in moons {
        println!("{:?}", m);
    }
}
