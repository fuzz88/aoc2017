use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;

type Tower = Box<Program>;

struct Program {
    name: String,
    weight: u32,
    children: Option<Vec<Tower>>,
}

type ParsedInput = HashMap<String, (u32, Vec<String>)>;

fn read_input(filename: &str) -> Result<Tower, Box<dyn error::Error>> {
    let splitted_lines: Vec<Vec<String>> = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    let mut parsed_input: ParsedInput = HashMap::new();

    for components in splitted_lines {
        let name = components[0].to_string();
        let weight: u32 = components[1][1..components[1].len() - 1].parse().unwrap();

        let mut children = vec![];

        if components.len() > 3 {
            let mut parsed_child;
            // child names with trailing comma
            for child in &components[3..components.len() - 1] {
                parsed_child = child[..child.len() - 1].to_string();
                children.push(parsed_child);
            }
            // last child name without comman
            let child = &components[components.len() - 1];
            parsed_child = child[..child.len()].to_string();
            children.push(parsed_child);
        }

        parsed_input.insert(name, (weight, children));
    }

    let mut all_children: Vec<_> = parsed_input
        .iter()
        .map(|(_name, values)| values.1.clone())
        .flatten()
        .collect();

    // for the binary search
    all_children.sort();

    let mut root_name = None;
    // n*log(n) to find root node
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
    // return root node if found, or throw an fatal error
    if let Some(root_name) = root_name {
        Ok(create_tower(&parsed_input, &root_name))
    } else {
        Err("fatal error: no root node is found")?
    }
}

fn create_tower(parsed_input: &ParsedInput, name: &str) -> Box<Program> {
    let program_data = parsed_input.get(name).unwrap();

    let weight = program_data.0;

    let mut tower = Box::new(Program {
        name: name.to_string(),
        weight,
        children: None,
    });

    let children_names = &program_data.1;

    let children: Vec<Tower> = children_names
        .iter()
        .map(|child_name| create_tower(parsed_input, child_name))
        .collect();

    if !children.is_empty() {
        tower.children = Some(children);
    }

    tower
}

fn part1(root: &Tower) -> &str {
    // What is the name of the bottom program?
    &root.name
}

fn part2(root: &Tower) -> u32 {
    // Given that exactly one program is the wrong weight,
    // what would its weight need to be to balance the entire tower?
    let mut disbalanced = vec![];
    inspect_weights(root, &mut disbalanced);
    disbalanced[0]
}

fn inspect_weights(tower: &Tower, disbalanced: &mut Vec<u32>) -> u32 {
    match &tower.children {
        None => tower.weight,
        Some(subtowers) => {
            // println!("{:?}", subtowers);
            let weights: Vec<_> = subtowers
                .iter()
                .map(|subtower| (inspect_weights(subtower, disbalanced), subtower.weight))
                .collect();
            let max_weight = weights.iter().max().unwrap();
            let min_weight = weights.iter().min().unwrap();
            if max_weight.0 != min_weight.0 {
                // println!("{:?}", weights);
                disbalanced.push(max_weight.1 - max_weight.0 + min_weight.0);
            }
            tower.weight + weights.iter().map(|subweights| subweights.0).sum::<u32>()
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 7: Recursive Circus ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
