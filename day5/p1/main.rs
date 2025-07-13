use std::fs;
use std::env;
use std::error;

fn read_input(filename: &str) -> Result<Vec<i32>, Box<dyn error::Error>> {
    let instructions = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    Ok(instructions)
}

fn main() -> Result<(), Box<dyn error::Error>>{
    println!("--- Day 5: A Maze of Twisty Trampolines, All Alike ---");

    let input_file = env::args().nth(1).ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{:?}", input_data);

    Ok(())
}
