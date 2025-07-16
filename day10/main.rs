use std::env;
use std::error;
use std::fs;

fn read_input(filename: &str) -> Result<Vec<u32>, Box<dyn error::Error>> {
    let lengths = fs::read_to_string(filename)?
        .split(',')
        .map(|l| l.trim_end().parse().unwrap())
        // .inspect(|l| println!("{}", l))
        .collect();

    Ok(lengths)
}

fn reverse(list: &Vec<u32>, position: usize, length: usize) -> &Vec<u32> {
    todo!()
}

fn part1(input: &Vec<u32>) -> u32 {
    let mut input = input.clone();
    let size = input.pop().unwrap() as usize;

    let mut list = vec![];
    (0..size).for_each(|num| list.push(num));

    println!("{:?}", list);

    0
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day10: Knot Hash ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));

    Ok(())
}
