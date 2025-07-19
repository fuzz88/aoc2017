use std::env;
use std::error;
use std::fs;

struct Generator<F: Fn(u64) -> bool> {
    current_value: u64,
    factor: u64,
    divider: u64,
    terminator: F,
}

impl<F: Fn(u64) -> bool> Iterator for Generator<F> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_value = (self.current_value) * self.factor % self.divider;

        while !(self.terminator)(self.current_value) {
            self.current_value = (self.current_value) * self.factor % self.divider;
        }

        Some(self.current_value)
    }
}

fn read_input(filename: &str) -> Result<Vec<u64>, Box<dyn error::Error>> {
    let init_values = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
        .collect();

    Ok(init_values)
}

fn part1(init_values: &[u64]) -> usize {
    let generator_a = Generator {
        current_value: init_values[0],
        factor: 16807,
        divider: 2147483647,
        terminator: |_| true,
    };
    let generator_b = Generator {
        current_value: init_values[1],
        factor: 48271,
        divider: 2147483647,
        terminator: |_| true,
    };

    generator_a
        .zip(generator_b)
        .take(40_000_000)
        .map(|(a, b)| {
            if ((a << 48) >> 48) == ((b << 48) >> 48) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(init_values: &[u64]) -> usize {
    let generator_a = Generator {
        current_value: init_values[0],
        factor: 16807,
        divider: 2147483647,
        terminator: |x| x.is_multiple_of(4),
    };
    let generator_b = Generator {
        current_value: init_values[1],
        factor: 48271,
        divider: 2147483647,
        terminator: |x| x.is_multiple_of(8),
    };

    generator_a
        .zip(generator_b)
        .take(5_000_000)
        .map(|(a, b)| {
            if ((a << 48) >> 48) == ((b << 48) >> 48) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day15: Dueling Generators ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
