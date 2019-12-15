// use std::io::{self, BufReader};
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut sum = 0;
    for line in handle.lines() {
        // println!("{}", line.unwrap());
        let i = line.unwrap().parse::<u32>().unwrap();
        sum += calc(i);
    }

    println!("{}", sum);
}

fn calc(x: u32) -> u32 {
    let tmp: u32 = x / 3;
    match tmp {
        0 => { 0 },
        1 => { 0 },
        _ => { tmp - 2 },
    }
}
