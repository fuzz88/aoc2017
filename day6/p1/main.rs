use std::collections::HashSet;
use std::env;
use std::error;
use std::fs;
use std::io;

type Banks = Vec<u32>;

fn read_input(filename: &str) -> Result<Banks, io::Error> {
    let banks = fs::read_to_string(filename)?
        .split_whitespace()
        .map(|bank| bank.parse().unwrap())
        .collect();

    Ok(banks)
}

fn cycler<F>(banks: &Banks, mut terminator: F) -> u32
where
    F: FnMut(&Banks, &mut HashSet<Banks>, &mut u32) -> bool,
{
    let mut steps = 0;
    let mut banks = banks.clone();
    let mut states = HashSet::<Banks>::new();

    loop {
        steps += 1;

        let mut bank_max = u32::MIN;
        let mut idx_max = 0;
        for (idx, &bank) in banks.iter().enumerate() {
            // if multiple max banks have the same number of blocks,
            // we reallocate only the first one.
            if bank > bank_max {
                bank_max = bank;
                idx_max = idx;
            }
        }

        // reallocation
        let amount = banks[idx_max];
        banks[idx_max] = 0;
        for shift in 1..=amount {
            let idx = (idx_max + shift as usize) % banks.len();
            banks[idx] += 1;
        }

        // println!("{:?}", banks);

        // check if the state is already seen.
        if terminator(&banks, &mut states, &mut steps) {
            return steps;
        }
    }
}

fn part1(banks: &Banks) -> u32 {
    cycler(banks, |banks, states, _| {
        if states.contains(banks) {
            return true;
        } else {
            states.insert(banks.clone());
            return false;
        }
    })
}

fn part2(banks: &Banks) -> u32 {
    let mut first_seen = true;
    let mut state_seen = vec![];
    let mut steps_seen = 0;

    cycler(banks, |banks, states, steps| {
        if !first_seen {
            if *banks == state_seen {
                // println!("{:?}", state_seen);
                *steps = *steps - steps_seen;
                return true;
            } else {
                return false;
            }
        } else {
            if states.contains(banks) {
                first_seen = false;
                state_seen = banks.clone();
                steps_seen = *steps;
                return false;
            } else {
                states.insert(banks.clone());
                return false;
            }
        }
    })
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 6: Memory Reallocation ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
