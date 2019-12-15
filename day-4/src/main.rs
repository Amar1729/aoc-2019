use std::io::{self, BufRead};

fn main() {
    let reader = io::stdin();
    let range: Vec<u32> =
        reader.lock()
        .lines().next().unwrap().unwrap()
        .split('-').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let result = calc(range[0], range[1]);
    println!("{}", result);
}

fn calc(low: u32, high: u32) -> usize {
    return (low+1..high).map(|x| check(x)).sum();
}

fn check(num: u32) -> usize {
    let cvec: Vec<char> = num.to_string().chars().collect();

    let mut c1 = 0; // check if at least two adjacent digits are equal

    let mut c = cvec[0];
    for i in 1..cvec.len() {
        if c == cvec[i] {
            c1 = 1;
        }

        // ensure digits are monotonic
        if cvec[i].to_digit(10).unwrap() < c.to_digit(10).unwrap() {
            return 0;
        }

        c = cvec[i];
    }

    return c1;
}
