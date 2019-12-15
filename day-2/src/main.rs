use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<u32> =
        input
        .split(',').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..100 {
        for j in 0..100 {
            let mut memory = numbers.clone();
            memory[1] = i;
            memory[2] = j;

            let output = calc(&mut memory);
            // println!("{}", output);
            if output == 19690720 {
                println!("{}", 100 * memory[1] + memory[2]);
            }
        }
    }
}

fn calc(numbers: &mut [u32]) -> u32 {
    let mut curr = 0;
    let mut opcode = numbers[curr];

    while opcode != 99 {
        match opcode {
            1 => {
                let x1 = numbers[numbers[curr+1] as usize];
                let x2 = numbers[numbers[curr+2] as usize];
                numbers[numbers[curr+3] as usize] = x1 + x2;
            },
            2 => {
                let x1 = numbers[numbers[curr+1] as usize];
                let x2 = numbers[numbers[curr+2] as usize];
                numbers[numbers[curr+3] as usize] = x1 * x2;
            },
            _ => { end(opcode, numbers); },
        }
        curr += 4;
        opcode = numbers[curr];
    }

    return numbers[0];
}

fn end(opcode: u32, numbers: &[u32]) {
    println!("{:?}", numbers);
    panic!("opcode: {}", opcode);
}
