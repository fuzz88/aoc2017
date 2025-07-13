use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;
use std::hash::Hash;

type Passphrase = Vec<String>;

fn read_input(filename: &str) -> Result<Vec<Passphrase>, Box<dyn error::Error>> {
    let passphrases = fs::read_to_string(filename)?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.to_string())
                .collect()
        })
        .collect();

    Ok(passphrases)
}

fn freqs<T: Eq + Hash + Clone>(items: &Vec<T>) -> HashMap<T, u32> {
    let mut freqs = HashMap::<T, u32>::new();

    for item in items {
        let freq = freqs.entry(item.clone()).or_insert(0);
        *freq += 1;
    }

    freqs
}

fn is_valid_1(pp: &&Passphrase) -> bool {
    freqs::<String>(pp).iter().all(|(_, freq)| *freq == 1)
}

fn part1(pps: &Vec<Passphrase>) -> usize {
    pps.iter().filter(is_valid_1).count()
}

fn equal_freqs<T: Eq + Hash>(freq1: &HashMap<T, u32>, freq2: &HashMap<T, u32>) -> bool {
    freq1.iter().all(|(ch, freq)| {
        let value = freq2.get(ch);
        value == Some(freq)
    }) && freq2.iter().all(|(ch, freq)| {
        let value = freq1.get(ch);
        value == Some(freq)
    })
}

fn is_valid_2(pp: &&Passphrase) -> bool {
    let word_freqs = freqs::<String>(pp);
    let uniq_words = word_freqs.iter().all(|(_, freq)| *freq == 1);
    if !uniq_words {
        return false;
    }
    let char_freqs: Vec<_> = pp
        .iter()
        .map(|word| freqs::<char>(&word.chars().collect()))
        .collect();

    // annagrams have the same char frequencies
    for (idx1, freq1) in char_freqs.iter().enumerate() {
        for (idx2, freq2) in char_freqs.iter().enumerate() {
            if idx1 != idx2 && equal_freqs::<char>(freq1, freq2) {
                return false;
            }
        }
    }
    true
}

fn part2(pps: &Vec<Passphrase>) -> usize {
    pps.iter().filter(is_valid_2).count()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 4: High-Entropy Passphrases ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    // println!("{input_data:?}");

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
