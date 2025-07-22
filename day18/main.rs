use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Operand {
    Register(char),
    Value(isize),
}

impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        match value.parse::<isize>() {
            Ok(integer) => Operand::Value(integer),
            Err(..) => Operand::Register(
                char::from_str(value)
                    .expect("expecting register name operand if we can't parse it as `isize`"),
            ),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Snd(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut components = value.split_whitespace();

        match components.next().unwrap() {
            // one operand
            "snd" => Instruction::Snd(Operand::from(components.next().unwrap())),
            "rcv" => Instruction::Rcv(Operand::from(components.next().unwrap())),
            // two operands
            "set" => Instruction::Set(
                Operand::from(components.next().unwrap()),
                Operand::from(components.next().unwrap()),
            ),
            "add" => Instruction::Add(
                Operand::from(components.next().unwrap()),
                Operand::from(components.next().unwrap()),
            ),
            "mul" => Instruction::Mul(
                Operand::from(components.next().unwrap()),
                Operand::from(components.next().unwrap()),
            ),
            "mod" => Instruction::Mod(
                Operand::from(components.next().unwrap()),
                Operand::from(components.next().unwrap()),
            ),
            "jgz" => Instruction::Jgz(
                Operand::from(components.next().unwrap()),
                Operand::from(components.next().unwrap()),
            ),
            instruction => unimplemented!("unknown instruction: {}", instruction),
        }
    }
}

type Registers = HashMap<char, isize>;

struct CPU<F>
where
    F: Fn(&Instruction, &Registers) -> Option<isize>,
{
    registers: Registers,
    pc: usize,
    trap: F,
}

impl<F: Fn(&Instruction, &Registers) -> Option<isize>> CPU<F> {
    fn new(trap: F) -> Self {
        CPU {
            registers: HashMap::new(),
            pc: 0,
            trap,
        }
    }
}

fn read_input(filename: &str) -> Result<Vec<Instruction>, Box<dyn error::Error>> {
    let instructions: Vec<Instruction> = fs::read_to_string(filename)?
        .lines()
        .map(Instruction::from)
        .collect();

    Ok(instructions)
}

fn part1(instructions: &[Instruction]) -> isize {
    let cpu = CPU::new(|instruction, registers| {
        println!("trapped: {:?}", instruction);
        None
    });

    0
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day18: Duet ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));

    Ok(())
}
