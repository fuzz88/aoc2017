#![feature(iter_map_windows)]

use std::cmp::Ordering;

fn input() -> &'static [u8] {
    include_bytes!("input.txt")
}

fn part1(seq: &[u8]) -> u32 {
    seq[..seq.len() - 1]
        .iter()
        .map_windows(|[a, b]| match a.cmp(b) {
            Ordering::Equal => (*a - 48) as u32,
            _ => 0,
        })
        .sum::<u32>()
        + if seq[0] == seq[seq.len() - 2] {
            (seq[0] - 48) as u32
        } else {
            0
        }
}

fn part2(seq: &[u8]) -> u32 {
    let mut result = 0;
    let seq = &seq[..seq.len() - 1];
    // println!("{}", seq.len());
    let shift = seq.len() / 2;

    for (idx, byte) in seq.iter().enumerate() {
        let digit = byte - 48; // ascii
        let next_digit = seq[(idx + shift) % seq.len()] - 48;
        if digit == next_digit {
            result += digit as u32;
        }
    }

    result
}

fn main() {
    println!("--- Day 1: Inverse Captcha ---");

    println!("{:?}", part1(input()));
    println!("{:?}", part2(input()));
}
