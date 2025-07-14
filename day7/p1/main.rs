use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Program {
    name: String,
    weight: u32,
    children: Option<Vec<Box<Program>>>,
}

type Tower = Box<Program>;
type ParsedInput = HashMap<String, (u32, Vec<String>)>;

fn read_input(filename: &str) -> Result<Tower, Box<dyn error::Error>> {
    let splitted_lines: Vec<Vec<String>> = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    let mut parsed_input: ParsedInput = HashMap::new();

    for components in splitted_lines {
        let weight: u32 = components[1][1..components[1].len() - 1].parse().unwrap();
        // println!("{} {}", name, weight);

        let mut children = vec![];

        if components.len() > 3 {
            let mut parsed_child;
            // child names with trailing comma
            for child in &components[3..components.len() - 1] {
                parsed_child = child[..child.len() - 1].to_string();
                children.push(parsed_child);
                // println!("{}", parsed_child);
            }
            // last child name without comman
            let child = &components[components.len() - 1];
            parsed_child = child[..child.len()].to_string();
            children.push(parsed_child);
            // println!("{}", parsed_child);
        }

        parsed_input.insert(components[0].clone(), (weight, children));
    }

    let mut all_children: Vec<_> = parsed_input
        .iter()
        .map(|(_name, values)| values.1.clone())
        .flatten()
        .collect();

    all_children.sort();

    let mut root_name = None;

    for (name, _) in &parsed_input {
        match all_children.binary_search(&name) {
            Ok(_) => {
                continue;
            }
            Err(_) => {
                root_name = Some(name);
                break;
            }
        }
    }
    if let Some(root_name) = root_name {
        let root = create_program(&parsed_input, &root_name);
        Ok(root)
    } else {
        Err("fatal error: no root node is found")?
    }

}

fn create_program(parsed_input: &ParsedInput, name: &str) -> Box<Program> {
    let program_data = parsed_input.get(name).unwrap();
    let mut program = Box::new(Program {
        name: name.to_string(),
        weight: program_data.0,
        children: None,
    });
    let children = program_data
        .1
        .iter()
        .map(|child_name| create_program(parsed_input, child_name))
        .collect();
    program.children = Some(children);
    program
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 7: Recursive Circus ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", input_data.name);

    Ok(())
}
