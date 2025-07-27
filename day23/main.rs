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

impl Operand {
    fn get_value(&self, registers: &mut Registers) -> isize {
        match self {
            Operand::Register(op_name) => *registers.entry(*op_name).or_insert(0),
            Operand::Value(op_value) => *op_value,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Set(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Jnz(Operand, Operand),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut components = value.split_whitespace();

        match components.next().unwrap() {
            // two operands
            "set" => Instruction::Set(
                Operand::from(components.next().unwrap()),
                Operand::from(components.next().unwrap()),
            ),
            "sub" => Instruction::Sub(
                Operand::from(components.next().unwrap()),
                Operand::from(components.next().unwrap()),
            ),
            "mul" => Instruction::Mul(
                Operand::from(components.next().unwrap()),
                Operand::from(components.next().unwrap()),
            ),
            "jnz" => Instruction::Jnz(
                Operand::from(components.next().unwrap()),
                Operand::from(components.next().unwrap()),
            ),
            instruction => unimplemented!("unknown instruction: {}", instruction),
        }
    }
}

impl Instruction {
    fn apply_mut<F>(&self, cpu: &mut CPU<F>)
    where
        F: FnMut(&Instruction),
    {
        match self {
            Instruction::Set(Operand::Register(reg_name), operand) => {
                *cpu.registers.entry(*reg_name).or_insert(0) =
                    operand.get_value(&mut cpu.registers);
                cpu.pc += 1;
            }
            Instruction::Sub(Operand::Register(reg_name), operand) => {
                *cpu.registers.entry(*reg_name).or_insert(0) -=
                    operand.get_value(&mut cpu.registers);
                cpu.pc += 1;
            }
            Instruction::Mul(Operand::Register(reg_name), operand) => {
                *cpu.registers.entry(*reg_name).or_insert(0) *=
                    operand.get_value(&mut cpu.registers);
                cpu.pc += 1;
            }
            Instruction::Jnz(operand1, operand2) => {
                let is_jmp = operand1.get_value(&mut cpu.registers) != 0;
                let offset = operand2.get_value(&mut cpu.registers);

                if is_jmp {
                    cpu.pc += offset;
                } else {
                    cpu.pc += 1;
                }
            }
            instruction => unimplemented!("unknown instruction: {:?}", instruction),
        }
    }
}

type Registers = HashMap<char, isize>;

#[allow(clippy::upper_case_acronyms)]
struct CPU<F>
where
    F: FnMut(&Instruction),
{
    registers: Registers,
    pc: isize,
    trap: F,
}

impl<F: FnMut(&Instruction)> CPU<F> {
    fn new(trap: F) -> Self {
        CPU {
            registers: HashMap::new(),
            pc: 0,
            trap,
        }
    }

    fn eval(&mut self, instructions: &[Instruction]) {
        loop {
            let next_instruction = &instructions[self.pc as usize];

            (self.trap)(next_instruction);

            next_instruction.apply_mut(self);

            if self.pc as usize >= instructions.len() {
                break;
            }
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

fn part1(instructions: &[Instruction]) -> usize {
    // How many times is the mul instruction invoked?
    let mut mul_count = 0;

    let mut cpu = CPU::new(|instruction| match instruction {
        Instruction::Mul(..) => mul_count += 1,
        _ => {}
    });

    cpu.eval(instructions);

    mul_count
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day23: Coprocessor Conflagration ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));

    Ok(())
}
