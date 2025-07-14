#![feature(stmt_expr_attributes)]
use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;

type Reg = String;

#[derive(Debug)]
enum OP {
    Inc(Reg, i32),
    Dec(Reg, i32),
}

impl OP {
    fn apply<F>(&self, get_value: F) -> (String, i32)
    where
        F: Fn(&str) -> i32,
    {
        match self {
            OP::Inc(reg, value) => (reg.to_string(), get_value(reg) + value),
            OP::Dec(reg, value) => (reg.to_string(), get_value(reg) - value),
        }
    }
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

impl COND {
    fn is_true<F>(&self, get_value: F) -> bool
    where
        F: Fn(&str) -> i32,
    {
        #[rustfmt::skip]
        match self {
            COND::GT(reg, value)  => { get_value(reg) >  *value },
            COND::GTE(reg, value) => { get_value(reg) >= *value },
            COND::LT(reg, value)  => { get_value(reg) <  *value },
            COND::LTE(reg, value) => { get_value(reg) <= *value },
            COND::EQ(reg, value)  => { get_value(reg) == *value },
            COND::NEQ(reg, value) => { get_value(reg) != *value },
        }
    }
}

#[derive(Debug)]
struct Instruction {
    operation: OP,
    condition: COND,
}

struct Computer {
    registers: HashMap<String, i32>,
    pc: usize,
    max_register_ever: i32,
}

impl Computer {
    fn new() -> Self {
        Computer {
            registers: HashMap::new(),
            pc: 0,
            max_register_ever: 0,
        }
    }

    fn get_reg(&self, reg: &str) -> i32 {
        *self.registers.get(reg).unwrap_or(&0)
    }

    fn eval(&mut self, program: &Vec<Instruction>) {
        let mut pc = self.pc;

        loop {
            match &program[pc] {
                Instruction {
                    operation: op,
                    condition: cond,
                } => {
                    // partial to bound self to get_reg
                    let get_value = |reg: &str| self.get_reg(reg);

                    if cond.is_true(get_value) {
                        let (reg, value) = op.apply(get_value);
                        let new_reg = self.registers.entry(reg).or_insert(0);
                        *new_reg = value;
                        self.max_register_ever =
                            self.max_register_ever.max(self.max_current_register());
                    }
                }
            };
            pc += 1;
            if pc == program.len() {
                break;
            }
        }
    }

    fn max_current_register(&self) -> i32 {
        *self
            .registers
            .values()
            .max()
            .expect("expecting values in registers")
    }
}

fn parse_line(line: &str) -> Instruction {
    let components: Vec<&str> = line.split_whitespace().collect();

    let operation_reg = components[0].to_string();
    let operation_arg = components[2].parse().unwrap();

    #[rustfmt::skip]
    let operation = match components[1] {
        "inc"     => OP::Inc(operation_reg, operation_arg),
        "dec"     => OP::Dec(operation_reg, operation_arg),
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
    // What is the largest value in any register
    // after completing the instructions in your puzzle input?
    let mut computer = Computer::new();
    computer.eval(program);
    computer.max_current_register()
}

fn part2(program: &Vec<Instruction>) -> i32 {
    // To be safe, the CPU also needs to know
    // the highest value held in any register during this process.
    let mut computer = Computer::new();
    computer.eval(program);
    computer.max_register_ever
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
    println!("{}", part2(&input_data));

    Ok(())
}
