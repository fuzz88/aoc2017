use std::env;
use std::error;
use std::fs;

type Component = (u32, u32);

fn read_input(filename: &str) -> Result<Vec<Component>, Box<dyn error::Error>> {
    let components = fs::read_to_string(filename)?
        .lines()
        .map(|line| {
            let mut ports = line.split("/");
            (
                ports.next().unwrap().parse().unwrap(),
                ports.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    Ok(components)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day24: Electromagnetic Moat ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{:?}", input_data);


    Ok(())
}
