use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashMap;

fn main() {

    // given by problem descr
    let length = 25 * 6;

    let f = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(f);

    // there's probably a better way (iterating over vectors?) to calculate these
    let mut minimum = 0;
    let mut result = 0;

    loop {
        let mut buf = vec![0u8; length];

        if let Err(_) = reader.read_exact(&mut buf) { break; }

        let output = calc(&buf);

        if minimum == 0 { minimum = output.0; }

        if output.0 > 0 && output.0 < minimum {
            minimum = output.0;
            result = output.1;
        }
    }

    println!("{}", result);
}

fn calc(pixels: &[u8]) -> (usize, usize) {

    let mut charmap = HashMap::new();

    //charmap.entry('0').or_insert(0);
    //charmap.entry('1').or_insert(0);
    //charmap.entry('2').or_insert(0);

    for c in pixels.iter().map(|b| *b as char) {
    // for c in pixels.map(|b| *b as char) {
        let count = charmap.entry(c).or_insert(0);
        *count += 1;
    }

    (
        *charmap.get(&'0').unwrap(),
        charmap.get(&'1').unwrap() * charmap.get(&'2').unwrap(),
    )
}
