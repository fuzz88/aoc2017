use std::collections::VecDeque;
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
    // Given your actual key string, how many squares are used?
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

            let used: u32 = list
                .chunks(16)
                // densify [256] -> [16]
                .map(|chunk| chunk.iter().copied().reduce(|acc, e| acc ^ e).unwrap())
                .map(|num| num.count_ones())
                .sum();

            used
        })
        .sum()
}

fn region_mark(disk: &mut [bool; 128 * 128], col: usize, row: usize) {
    let mut to_process = VecDeque::new();

    to_process.push_back((col, row));

    while let Some((col, row)) = to_process.pop_front() {
        disk[row * 128 + col] = false;

        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            // https://t.me/bminaiev_blog/59
            let nrow = row.overflowing_add(dr).0;
            let ncol = col.overflowing_add(dc).0;

            if nrow < 128 && ncol < 128 && disk[nrow * 128 + ncol] {
                to_process.push_back((ncol, nrow));
            }
        }
    }
}

fn part2(input: &String) -> u32 {
    // How many regions are present given your key string?
    let mut disk = [false; 128 * 128];

    (0..128).for_each(|idx| {
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

        let mut row: Vec<u8> = list
            .chunks(16)
            // densify [256] -> [16]
            .map(|chunk| chunk.iter().copied().reduce(|acc, e| acc ^ e).unwrap())
            .collect();

        let mut col_n = 128;
        let mut bits = 0; // unused 0, but must be initialized at this point.

        while col_n != 0 {
            if col_n % 8 == 0 {
                bits = row.pop().unwrap();
            }
            disk[idx * 128 + col_n - 1] = (bits != 0) && bits & 1 == 1;
            if bits != 0 {
                bits = bits >> 1;
            }
            col_n -= 1;
        }
    });

    let mut regions_count = 0;

    for row in 0..128 {
        for col in 0..128 {
            if disk[row * 128 + col] {
                regions_count += 1;
                region_mark(&mut disk, col, row);
            }
        }
    }

    regions_count
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day14: Disk Defragmentation ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
