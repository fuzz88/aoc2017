use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;

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

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 4: High-Entropy Passphrases ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{input_data:?}");

    Ok(())
}
