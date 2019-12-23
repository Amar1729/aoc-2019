#![allow(dead_code)]

use std::fs;
use std::io::{self, BufRead};

struct ParamMode {
    code: usize,
    modes: Vec<usize>,
}

impl ParamMode {
    fn new(opcode: u32) -> ParamMode {
        let mut cvec: Vec<usize> = opcode.to_string().chars().rev()
            .map(|c| c.to_digit(10).unwrap())
            .map(|x| x as usize)
            .collect();

        if cvec.len() == 1 { cvec.push(0); }

        // println!("new pmode: {:?}", opcode);

        // let _code: Vec<char> 
        let code = format!("{}{}", cvec[1], cvec[0]).parse::<usize>().unwrap();
        cvec.remove(0);
        cvec.remove(0);

        match code {
            1 => {
                // gross
                let mut modes = cvec.clone();
                while modes.len() < 3 {
                    modes.push(0);
                }
                ParamMode { code: code, modes: modes }
            },
            2 => {
                // gross
                let mut modes = cvec.clone();
                while modes.len() < 3 {
                    modes.push(0);
                }
                ParamMode { code: code, modes: modes }
            },
            3 => {
                // gross
                let mut modes = cvec.clone();
                while modes.len() < 1 {
                    // ALERT - opcode 3 uses what is essentially immediate mode to index to storage
                    // location
                    modes.push(1);
                }
                ParamMode { code: code, modes: modes }
            },
            4 => {
                // gross
                let mut modes = cvec.clone();
                while modes.len() < 1 {
                    modes.push(0);
                }
                ParamMode { code: code, modes: modes }
            },
            5 => {
                let mut modes = cvec;
                while modes.len() < 2 {
                    modes.push(0);
                }
                ParamMode { code: code, modes: modes }
            },
            6 => {
                let mut modes = cvec;
                while modes.len() < 2 {
                    modes.push(0);
                }
                ParamMode { code: code, modes: modes }
            },
            7 => {
                let mut modes = cvec;
                while modes.len() < 2 {
                    modes.push(0);
                }
                if modes.len() < 3 { modes.push(1); } // see opcode 3 alert
                ParamMode { code: code, modes: modes }
            },
            8 => {
                let mut modes = cvec;
                while modes.len() < 2 {
                    modes.push(0);
                }
                if modes.len() < 3 { modes.push(1); } // see opcode 3 alert
                ParamMode { code: code, modes: modes }
            },
            99 => {
                let modes = Vec::new();
                ParamMode { code: code, modes: modes }
            },
            _ => {
                panic!("Unimpl: {}", code);
            },
        }
    }
}

fn main() {
    let mut memory: Vec<i32> =
        fs::read_to_string("mem.txt").unwrap()
        .split(',').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    calc(&mut memory);
}

/* dont know how to instrument the rest of program with this function?
fn amplifier(memory: &mut [i32], phase: usize) -> i32 {
    // takes memory and phase setting
    // will read one input from stdin, and give one output by running our intcode computer

    let reader = io::stdin();
    let input_instruction =
        reader.lock()
        .lines().next().unwrap().unwrap()
        .parse::<i32>().unwrap();

    return calc(memory);
}
*/

fn calc(numbers: &mut [i32]) -> i32 {
    let mut curr = 0;
    let mut pmode = ParamMode::new(numbers[curr] as u32); // is i32 -> u32 safe?

    while pmode.code != 99 {

        let gp = |p: usize| -> i32 {
            if pmode.modes[p] == 0 {
                numbers[numbers[curr+p+1] as usize]
            } else {
                numbers[curr+p+1]
            }
        };

        match pmode.code {
            1 => {
                let x1 = gp(0);
                let x2 = gp(1);
                // println!("{} + {}", x1, x2);
                numbers[numbers[curr+3] as usize] = x1 + x2;
                curr += 4;
            },
            2 => {
                let x1 = gp(0);
                let x2 = gp(1);
                numbers[numbers[curr+3] as usize] = x1 * x2;
                curr += 4;
            },
            3 => {
                // let store = numbers[numbers[curr+1] as usize];
                let store = gp(0);

                let reader = io::stdin();
                let input =
                    reader.lock()
                    .lines().next().unwrap().unwrap()
                    .parse::<i32>().unwrap();

                numbers[store as usize] = input;
                // println!("[{}] = {}", store, input);
                curr += 2;
            },
            4 => {
                let output = gp(0);
                println!(">{}", output);

                // only panic for '4' instructions that are not the last one
                // i.e. remove this after debugging
                // if output != 0 { diagnostic_failure(curr, numbers) }

                curr += 2;
            },
            5 => {
                if gp(0) != 0 {
                    curr = gp(1) as usize;
                } else {
                    curr += 3;
                }
            },
            6 => {
                if gp(0) == 0 {
                    curr = gp(1) as usize;
                } else {
                    curr += 3;
                }
            },
            7 => {
                numbers[gp(2) as usize] = if gp(0) < gp(1) {
                    1
                } else {
                    0
                };
                curr += 4;
            },
            8 => {
                numbers[gp(2) as usize] = if gp(0) == gp(1) {
                    1
                } else {
                    0
                };
                curr += 4;
            },
            // exit on unimplemented
            _ => { end(pmode.code, numbers); },
        }
        pmode = ParamMode::new(numbers[curr] as u32);
    }

    return numbers[0];
}

fn diagnostic_failure(curr: usize, numbers: &[i32]) {
    // print diagnostic info
    println!("rax: {}", curr);

    // table header
    print!("        ");
    for i in 0..10 { print!("{:>8}", i); }
    print!("\n        ");
    for _ in 0..10 { print!("{:-<1$}", "", 8); }

    // elements
    let mut c = 0;
    for elem in numbers {
        if c % 10 == 0 { print!("\n{:>6} |", c); }
        c += 1;
        print!("{:>8}", elem);
    }

    panic!();
}

fn end(opcode: usize, numbers: &[i32]) {
    println!("{:?}", numbers);
    panic!("opcode: {}", opcode);
}
