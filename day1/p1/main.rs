use std::env;
use std::error;
use std::fs;

fn read_input(filename: &str) -> Result<String, Box<dyn error::Error>> {
    let content = fs::read_to_string(filename)?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 1: Inverse Captcha ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file name as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", input_data);

    Ok(())
}
