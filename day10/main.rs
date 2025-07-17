use std::env;
use std::error;
use std::fs;

fn read_input(filename: &str) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let lengths = fs::read_to_string(filename)?
        .split(',')
        .map(|l| l.trim_end().parse().unwrap())
        .collect();

    Ok(lengths)
}

fn reverse(list: &mut Vec<u8>, position: usize, length: usize) {
    let end_pos = position + length - 1;

    let mut l = position % list.len();
    let mut h = end_pos % list.len();

    while l != (position + length / 2) % list.len() {
        if l != h {
            // swap
            list[l] = list[h] ^ list[l];
            list[h] = list[l] ^ list[h];
            list[l] = list[h] ^ list[l]
        }
        l += 1;
        l = l % list.len();
        if h == 0 {
            h = list.len();
        }
        h -= 1;
    }
}

fn part1(input: &Vec<usize>) -> u16 {
    let mut list = vec![];
    (0..=255).for_each(|num| list.push(num as u8));

    let mut skip = 0;
    let mut position = 0;

    // one round
    for length in input {
        reverse(&mut list, position, *length);
        position += (*length + skip) % list.len();
        skip += 1;
    }

    list[0] as u16 * list[1] as u16
}

fn part2(input: &[u8]) -> String {
    let mut lengths: Vec<usize> = input[0..input.len() - 1]
        .iter()
        .copied()
        .map(|v| v as usize)
        .collect();

    lengths.extend([17, 31, 73, 47, 23]);

    let mut list = vec![];
    (0..=255).for_each(|num| list.push(num as u8));

    let mut position = 0;
    let mut skip = 0;

    // 64 rounds
    (0..64).for_each(|_| {
        for length in &lengths {
            reverse(&mut list, position, *length);
            position += (*length + skip) % list.len();
            skip += 1;
        }
    });

    let hash = list
        .chunks(16)
        // densify [256] -> [16]
        .map(|chunk| chunk.iter().copied().reduce(|acc, e| acc ^ e).unwrap())
        // represent as 32-char hexidecimal string
        .map(|num| format!("{num:02x}"))
        .collect::<Vec<String>>()
        .join("");

    hash
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day10: Knot Hash ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(fs::read_to_string(&input_file)?.as_bytes()));

    Ok(())
}
