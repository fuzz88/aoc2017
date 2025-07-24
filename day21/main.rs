use std::env;
use std::error;
use std::fs;

type Rule = (Vec<u8>, Vec<u8>);

fn read_input(filename: &str) -> Result<Vec<Rule>, Box<dyn error::Error>> {
    let rules = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.split(" => ").map(|s| s.as_bytes().to_owned()))
        .map(|mut bytes| (bytes.next().unwrap(), bytes.next().unwrap()))
        .collect();

    Ok(rules)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day21: Fractal Art ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{:?}", input_data);

    Ok(())
}
