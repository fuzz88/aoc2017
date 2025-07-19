use std::collections::VecDeque;
use std::env;
use std::error;
use std::fs;

#[derive(Debug)]
struct BitField128 {
    arr: [u8; 2048],
}

impl BitField128 {
    fn from_vec(numbers: &[u8]) -> Self {
        assert!(numbers.len() == 2048);

        let mut arr: [u8; 2048] = [0; 2048];

        for idx in 0..2048 {
            arr[idx] = numbers[idx];
        }

        BitField128 { arr }
    }

    fn is_used(&self, col: usize, row: usize) -> bool {
        let idx = row * 128 + col;
        let n_idx = idx / 8;
        let shift = 7 - idx % 8;

        self.arr[n_idx] & (1 << shift) != 0
    }

    fn set_unused(&mut self, col: usize, row: usize) {
        let idx = row * 128 + col;
        let n_idx = idx / 8;
        let shift = 7 - idx % 8;

        self.arr[n_idx] = self.arr[n_idx] & !(1 << shift);
    }
}

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
            list[l] = list[l] ^ list[h];
            list[h] = list[l] ^ list[h];
            list[l] = list[l] ^ list[h]
        }
        l += 1;
        l = l % list.len();
        if h == 0 {
            h = list.len();
        }
        h -= 1;
    }
}

fn calculate_sparse_hash(input: &str, idx: usize) -> Vec<u8> {
    let mut to_hash = input.to_string();
    to_hash.push('-');
    to_hash.push_str(&idx.to_string());

    let mut lengths: Vec<usize> = to_hash
        .as_bytes()
        .iter()
        .copied()
        .map(|v| v as usize)
        .collect();

    lengths.extend([17, 31, 73, 47, 23]);

    let mut hash = vec![];
    (0..=255).for_each(|num| hash.push(num as u8));

    let mut position = 0;
    let mut skip = 0;

    // 64 rounds
    (0..64).for_each(|_| {
        for length in &lengths {
            reverse(&mut hash, position, *length);
            position += (*length + skip) % hash.len();
            skip += 1;
        }
    });

    hash
}

fn part1(input: &str) -> u32 {
    // Given your actual key string, how many squares are used?
    (0..128)
        .map(|idx| {
            let sparse_hash = calculate_sparse_hash(input, idx);

            let used: u32 = sparse_hash
                .chunks(16)
                // densify [256] -> [16]
                .map(|chunk| chunk.iter().copied().reduce(|acc, e| acc ^ e).unwrap())
                .map(|num| num.count_ones())
                .sum();

            used
        })
        .sum()
}

fn region_mark(disk: &mut BitField128, col: usize, row: usize) {
    let mut to_process = VecDeque::new();

    to_process.push_back((col, row));

    while let Some((col, row)) = to_process.pop_front() {
        disk.set_unused(col, row);

        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            // https://t.me/bminaiev_blog/59
            let nrow = row.overflowing_add(dr).0;
            let ncol = col.overflowing_add(dc).0;

            if nrow < 128 && ncol < 128 && disk.is_used(ncol, nrow) {
                to_process.push_back((ncol, nrow));
            }
        }
    }
}

fn part2(input: &str) -> u32 {
    // How many regions are present given your key string?
    //
    let mut disk = Vec::<u8>::new();

    (0..128).for_each(|idx| {
        let sparse_hash = calculate_sparse_hash(input, idx);

        sparse_hash
            .chunks(16)
            // densify [256] -> [16]
            .map(|chunk| chunk.iter().copied().reduce(|acc, e| acc ^ e).unwrap())
            .for_each(|num| disk.push(num));
    });

    let mut disk = BitField128::from_vec(&disk);

    let mut regions_count = 0;

    for row in 0..128 {
        for col in 0..128 {
            if disk.is_used(col, row) {
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
