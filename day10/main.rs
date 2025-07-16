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

fn reverse(list: &Vec<u32>, position: u32, length: u32) -> &Vec<u32> {
    println!("{:?}", list);
    println!("{} {}", position, length);
    todo!()
}

fn part1(input: &Vec<u32>) -> u32 {
    let mut input = input.clone();
    let size = input.pop().unwrap() as usize;

    let mut list = vec![];
    (0..size).for_each(|num| list.push(num as u32));
    println!("{:?}", list);

    let mut skip = 0;
    let mut position = 0;

    for length in input {
        list = reverse(&list, position, length).to_vec();
        position += (length + skip) % list.len() as u32;
        skip += 1;
    }

    list[0..=1].iter().copied().product()
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
