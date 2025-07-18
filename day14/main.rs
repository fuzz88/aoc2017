use std::env;
use std::error;
use std::fs;

fn read_input(filename: &str) -> Result<String, Box<dyn error::Error>> {
    Ok(fs::read_to_string(filename)?.trim_end().to_string())
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

fn part1(input: &String) -> u32 {
    (0..128)
        .map(|idx| {
            let mut to_hash = input.clone();
            to_hash.push('-');
            to_hash.push_str(&idx.to_string());

            let mut lengths: Vec<usize> = to_hash
                .as_bytes()
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

            let used: u32 = list.chunks(16)
                // densify [256] -> [16]
                .map(|chunk| chunk.iter().copied().reduce(|acc, e| acc ^ e).unwrap())
                .map(|num| num.count_ones())
                .sum();

            used
        })
        .sum()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day14: Disk Defragmentation ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));

    Ok(())
}
