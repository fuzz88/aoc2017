use std::env;
use std::error;
use std::fs;

type Component = (u32, u32);

fn read_input(filename: &str) -> Result<Vec<Component>, Box<dyn error::Error>> {
    let components = fs::read_to_string(filename)?
        .lines()
        .map(|line| {
            let mut ports = line.split("/");
            (
                ports.next().unwrap().parse().unwrap(),
                ports.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    Ok(components)
}

fn make_pair(component1: &Component, component2: &Component) -> Option<Component> {
    if component1.1 == component2.0 {
        return Some(*component2);
    }
    if component1.1 == component2.1 {
        return Some((component2.1, component2.0));
    }

    None
}

fn find_strongest_bridge(bridge: Vec<Component>, components: Vec<Component>, strength: u32) -> u32 {
    let last = bridge[bridge.len() - 1];

    let mut strengths = vec![];

    for (idx, comp) in components.iter().enumerate() {
        if let Some(comp) = make_pair(&last, comp) {
            let mut next_components = components.clone();
            next_components.remove(idx);
            let mut next_bridge = bridge.clone();
            next_bridge.push(comp);
            strengths.push(find_strongest_bridge(
                next_bridge,
                next_components,
                strength + comp.0 + comp.1,
            ));
        }
    }

    *strengths.iter().max().unwrap_or(&strength)
}

fn find_longest_bridge(
    bridge: Vec<Component>,
    components: Vec<Component>,
    strength: u32,
    length: u32,
) -> (u32, u32) {
    let last = bridge[bridge.len() - 1];

    let mut strengths = vec![];

    for (idx, comp) in components.iter().enumerate() {
        if let Some(comp) = make_pair(&last, comp) {
            let mut next_components = components.clone();
            next_components.remove(idx);
            let mut next_bridge = bridge.clone();
            next_bridge.push(comp);
            strengths.push(find_longest_bridge(
                next_bridge,
                next_components,
                strength + comp.0 + comp.1,
                length + 1,
            ));
        }
    }

    if let Some((length, strength)) = strengths.iter().max() {
        return (*length, *strength);
    }
    (length, strength)
}

fn part1(components: &[Component]) -> u32 {
    // What is the strength of the strongest bridge
    // you can make with the components you have available?
    let bridge = vec![(0, 0)];

    find_strongest_bridge(bridge, components.to_vec(), 0)
}

fn part2(components: &[Component]) -> u32 {
    // What is the strength of the longest bridge you can make?
    // If you can make multiple bridges of the longest length, pick the strongest one.
    let bridge = vec![(0, 0)];

    find_longest_bridge(bridge, components.to_vec(), 0, 0).1
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day24: Electromagnetic Moat ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
