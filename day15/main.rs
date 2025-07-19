use std::env;
use std::error;
use std::fs;

#[derive(Debug)]
struct Generator {
    current_value: u32,
    factor: u32,
    divider: u32,
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_value = self.current_value * self.factor % self.divider;
        Some(self.current_value)
    }
}

fn read_input(filename: &str) -> Result<Vec<u32>, Box<dyn error::Error>> {
    let init_values = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
        .collect();

    Ok(init_values)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day15: Dueling Generators ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    let mut generator_a = Generator {
        current_value: input_data[0],
        factor: 16807,
        divider: 2147483647,
    };
    let mut generator_b = Generator {
        current_value: input_data[1],
        factor: 48271,
        divider: 2147483647,
    };

    println!(
        "{} {}",
        generator_a.next().unwrap(),
        generator_b.next().unwrap()
    );

    Ok(())
}
