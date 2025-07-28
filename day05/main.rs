use std::{env, error, fs, io};

fn read_input(filename: &str) -> Result<Vec<i32>, io::Error> {
    let instructions = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    Ok(instructions)
}

fn evaluate<F>(instructions: &mut Vec<i32>, mut modify: F) -> u32
where
    F: FnMut(&mut Vec<i32>, i32, i32),
{
    let mut steps = 0;
    let mut pc: i32 = 0;

    loop {
        let jump = instructions[pc as usize];
        modify(instructions, pc, jump);
        pc += jump;
        assert!(pc >= 0, "invalid instructions");

        steps += 1;

        if pc as usize >= instructions.len() {
            break steps;
        }
    }
}

fn part1(mut instructions: Vec<i32>) -> u32 {
    evaluate(&mut instructions, |instructions, pc, _jump| {
        instructions[pc as usize] += 1
    })
}

fn part2(mut instructions: Vec<i32>) -> u32 {
    evaluate(&mut instructions, |instructions, pc, jump| {
        if jump >= 3 {
            instructions[pc as usize] -= 1;
        } else {
            instructions[pc as usize] += 1;
        }
    })
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 5: A Maze of Twisty Trampolines, All Alike ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    // println!("{:?}", input_data);

    println!("{}", part1(input_data.clone()));
    println!("{}", part2(input_data.clone()));

    Ok(())
}
