use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const LENGTH: usize = 25 * 6;

struct Img {
    pixels: [usize; LENGTH],
}

impl Img {
    fn new() -> Img {
        let px = [2usize; LENGTH];

        Img {
            pixels: px,
        }
    }

    fn update(&mut self, px: &[u32]) {
        for i in 0..LENGTH {
            match &self.pixels[i] {
                2 => {
                    self.pixels[i] = px[i] as usize;
                },
                _ => (),
            }
        }
    }
}

fn main() {

    let f = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(f);

    let mut img = Img::new();

    loop {
        let mut buf = vec![0u8; LENGTH];

        if let Err(_) = reader.read_exact(&mut buf) { break; }

        let px: Vec<u32> = buf.iter().map(|b| *b as char).map(|c| c.to_digit(10).unwrap()).collect();

        img.update(&px);
    }

    // println!("{:?}", img);

    for c in 0..LENGTH {
        if c % 25 == 0 { print!("\n"); }

        if img.pixels[c] == 1 {
            print!("1");
        } else {
            print!(" ");
        }
        // print!("{}", img.pixels[c]);
    }
}
