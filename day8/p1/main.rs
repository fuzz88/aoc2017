use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;

type Reg = String;

#[derive(Debug)]
enum OP {
    Inc(i32),
    Dec(i32),
}

#[derive(Debug)]
enum COND {
    GT(Reg, i32),
    GTE(Reg, i32),
    LT(Reg, i32),
    LTE(Reg, i32),
    EQ(Reg, i32),
    NEQ(Reg, i32),
}

#[derive(Debug)]
struct Instruction {
    register: Reg,
    operation: OP,
    condition: COND,
}

struct Computer {
    registers: HashMap<String, i32>,
    pc: usize,
}

impl Computer {
    fn new() -> Self {
        Computer {
            registers: HashMap::new(),
            pc: 0,
        }
    }

    fn eval(&mut self, program: &Vec<Instruction>) {}

    fn max_register(&self) -> i32 {
        *self
            .registers
            .values()
            .max()
            .expect("expecting values in registers")
    }
}

fn parse_line(line: &str) -> Instruction {
    let components: Vec<&str> = line.split_whitespace().collect();

    let register = components[0].to_string();

    let operation_arg = components[2].parse().unwrap();

    #[rustfmt::skip]
    let operation = match components[1] {
        "inc"     => OP::Inc(operation_arg),
        "dec"     => OP::Dec(operation_arg),
        operation => unimplemented!("unknown operation: {}", operation),
    };

    let condition_reg = components[4].parse().unwrap();
    let condition_arg = components[6].parse().unwrap();

    #[rustfmt::skip]
    let condition = match components[5] {
        ">"       => COND::GT(condition_reg, condition_arg),
        ">="      => COND::GTE(condition_reg, condition_arg),
        "<"       => COND::LT(condition_reg, condition_arg),
        "<="      => COND::LTE(condition_reg, condition_arg),
        "=="      => COND::EQ(condition_reg, condition_arg),
        "!="      => COND::NEQ(condition_reg, condition_arg),
        condition => unimplemented!("unknown condition: {}", condition),
    };

    Instruction {
        register,
        operation,
        condition,
    }
}

fn read_input(filename: &str) -> Result<Vec<Instruction>, Box<dyn error::Error>> {
    let instructions = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_line(line))
        .collect();

    Ok(instructions)
}

fn part1(program: &Vec<Instruction>) -> i32 {
    let mut computer = Computer::new();
    computer.eval(program);
    computer.max_register()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 8: I Heard You Like Registers ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;
    // println!("{}", input_file);

    let input_data = read_input(&input_file)?;
    // println!("{:?}", input_data);

    println!("{}", part1(&input_data));

    Ok(())
}
