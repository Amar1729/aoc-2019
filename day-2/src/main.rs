use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut numbers: Vec<u32> =
        input
        .split(',').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut curr = 0;
    loop {
        // println!("{:?}", numbers);

        calc(curr, &mut numbers);
        curr += 4;
    }
}

fn calc(curr: usize, numbers: &mut [u32]) {
    let get_pos = |pos: usize| -> u32 { numbers[numbers[pos] as usize] };

    let opcode = numbers[curr];
    match opcode {
        1 => {
            let x1 = get_pos(curr+1);
            let x2 = get_pos(curr+2);
            numbers[numbers[curr+3] as usize] = x1 + x2;
        },
        2 => {
            let x1 = get_pos(curr+1);
            let x2 = get_pos(curr+2);
            numbers[numbers[curr+3] as usize] = x1 * x2;
        },
        _ => { end(opcode, numbers); },
    }
}

fn end(opcode: u32, numbers: &[u32]) {
    println!("{:?}", numbers);
    panic!("opcode: {}", opcode);
}
