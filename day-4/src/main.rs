use std::io::{self, BufRead};

struct Adj {
    c: char,
    count: usize,
    adj: Vec<usize>,
}

impl Adj {
    fn new(c: char) -> Adj {
        Adj {
            c: c,
            count: 1,
            adj: Vec::new(),
        }
    }

    fn update(&mut self, c: char) {
        if c == self.c {
            self.count += 1;
        } else {
            self.adj.push(self.count);
            self.c = c;
            self.count = 1;
        }
    }

    fn end(&mut self) {
        self.adj.push(self.count);
    }

    /*
     * I'd like to return self.adj here, but i don't know lifetimes well enough
    fn finals(&self) -> &[usize] {
        self.adj.as_slice()
    }
    */

    fn check_value(&self, i: usize) -> usize {
        for n in &self.adj {
            if &i == n { return 1 as usize; }
        }
        return 0 as usize;
    }
}

fn main() {
    let reader = io::stdin();
    let range: Vec<u32> =
        reader.lock()
        .lines().next().unwrap().unwrap()
        .split('-').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    // let result = calc(range[0], range[1]);
    let result: usize = (range[0]..range[1]).map(|x| calc(x)).sum();
    println!("{}", result);
}

fn calc(num: u32) -> usize {
    let cvec: Vec<char> = num.to_string().chars().collect();

    let mut adj = Adj::new(cvec[0]);

    for i in 1..cvec.len() {
        // ensure digits are monotonic
        if cvec[i].to_digit(10).unwrap() < adj.c.to_digit(10).unwrap() {
            return 0;
        }

        adj.update(cvec[i]);
    }

    adj.end();

    return adj.check_value(2);
}
