use std::env;
use std::error;
use std::fs;

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn read_input(filename: &str) -> Result<Vec<Vec<u32>>, Box<dyn error::Error>> {
    let spreadsheet = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_line(line))
        .collect();

    Ok(spreadsheet)
}

fn part1(table: &Vec<Vec<u32>>) -> u32 {
    table
        .iter()
        // .inspect(|row| println!("{:?}", row))
        .map(|row| {
            let (min, max) = row
                .iter()
                .fold((row[0], row[0]), |(min, max), &x| (min.min(x), max.max(x)));
            max - min
        })
        .sum()
}

fn get_goal(row: &Vec<u32>) -> u32 {
    for a in row {
        for b in row {
            if a > b && a % b == 0 {
                return a / b;
            }
        }
    }
    unreachable!("must be always a goal");
}

fn part2(table: &Vec<Vec<u32>>) -> u32 {
    table.iter().map(|row| get_goal(row)).sum()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 2: Corruption Checksum ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    // println!("{input_file}");

    let input_data = read_input(&input_file)?;

    // println!("{:?}", input_data);

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
