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

impl Instruction {
    fn apply_mut<F>(&self, cpu: &mut CPU<F>)
    where
        F: FnMut(&Instruction, &Registers) -> Option<isize>,
    {
        match self {
            Instruction::Set(register, operand) => {
                if let Operand::Register(reg_name) = register {
                    let value = match operand {
                        Operand::Register(op_name) => *cpu.registers.entry(*op_name).or_insert(0),
                        Operand::Value(op_value) => *op_value,
                    };

                    let register = cpu.registers.entry(*reg_name).or_insert(0);
                    *register = value;
                    cpu.pc += 1;
                }
            }
            Instruction::Add(register, operand) => {
                if let Operand::Register(reg_name) = register {
                    let value = match operand {
                        Operand::Register(op_name) => *cpu.registers.entry(*op_name).or_insert(0),
                        Operand::Value(op_value) => *op_value,
                    };

                    let register = cpu.registers.entry(*reg_name).or_insert(0);
                    *register += value;
                    cpu.pc += 1;
                }
            }
            Instruction::Mul(register, operand) => {
                if let Operand::Register(reg_name) = register {
                    let value = match operand {
                        Operand::Register(op_name) => *cpu.registers.entry(*op_name).or_insert(0),
                        Operand::Value(op_value) => *op_value,
                    };

                    let register = cpu.registers.entry(*reg_name).or_insert(0);
                    *register *= value;
                    cpu.pc += 1;
                }
            }
            Instruction::Mod(register, operand) => {
                if let Operand::Register(reg_name) = register {
                    let value = match operand {
                        Operand::Register(op_name) => *cpu.registers.entry(*op_name).or_insert(0),
                        Operand::Value(op_value) => *op_value,
                    };

                    let register = cpu.registers.entry(*reg_name).or_insert(0);
                    *register %= value;
                    cpu.pc += 1;
                }
            }
            Instruction::Snd(operand) => {
                let value = match operand {
                    Operand::Register(op_name) => *cpu.registers.entry(*op_name).or_insert(0),
                    Operand::Value(op_value) => *op_value,
                };
                let register = cpu.registers.entry('~').or_insert(0);
                *register = value;
                cpu.pc += 1;
            }
            Instruction::Rcv(..) => {
                // processed via trap
                cpu.pc += 1;
            }
            Instruction::Jgz(operand1, operand2) => {
                let is_jmp = match operand1 {
                    Operand::Register(op_name) => *cpu.registers.entry(*op_name).or_insert(0) > 0,
                    Operand::Value(op_value) => *op_value > 0,
                };
                let offset = match operand2 {
                    Operand::Register(op_name) => *cpu.registers.entry(*op_name).or_insert(0),
                    Operand::Value(op_value) => *op_value,
                };
                if is_jmp {
                    cpu.pc += offset;
                } else {
                    cpu.pc += 1;
                }
            }
        }
    }
}

type Registers = HashMap<char, isize>;

struct CPU<F>
where
    F: FnMut(&Instruction, &Registers) -> Option<isize>,
{
    registers: Registers,
    pc: isize,
    trap: F,
}

impl<F: FnMut(&Instruction, &Registers) -> Option<isize>> CPU<F> {
    fn new(trap: F) -> Self {
        CPU {
            registers: HashMap::new(),
            pc: 0,
            trap,
        }
    }

    fn eval(&mut self, instructions: &[Instruction]) -> isize {
        loop {
            let next_instruction = &instructions[self.pc as usize];

            if let Some(value) = (self.trap)(next_instruction, &self.registers) {
                return value;
            };

            next_instruction.apply_mut(self);

            // println!("{}", self.pc);
            // println!("{:?}", next_instruction);
            // std::thread::sleep(std::time::Duration::from_millis(2000));

            if self.pc as usize >= instructions.len() {
                return 0;
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

fn part1(instructions: &[Instruction]) -> isize {
    // What is the value of the recovered frequency
    // (the value of the most recently played sound)
    // the first time a rcv instruction is executed with a non-zero value?
    let mut cpu = CPU::new(|instruction, registers| {
        match instruction {
            Instruction::Rcv(operand) => match operand {
                Operand::Register(name) => {
                    if *registers.get(name).unwrap() != 0 {
                        return Some(*registers.get(&'~').unwrap());
                    }
                }
                Operand::Value(value) => {
                    if *value != 0 {
                        return Some(*registers.get(&'~').unwrap());
                    }
                }
            },
            _ => {}
        };
        None
    });

    cpu.eval(instructions)
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
