use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;

type Firewall = HashMap<u32, u32>; // depth and range

fn parse_line(line: &str) -> (u32, u32) {
    let mut components = line.split(": ").map(|num| num.parse().unwrap());
    (components.next().unwrap(), components.next().unwrap())
}

fn read_input(filename: &str) -> Result<Firewall, Box<dyn error::Error>> {
    let rules = fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect();

    Ok(rules)
}

fn part1(firewall: &Firewall) -> u32 {
    // Given the details of the firewall you've recorded,
    // if you leave immediately, what is the severity of your whole trip?
    let mut severity = 0;
    let max_layer = firewall.keys().max().unwrap();

    for idx in 0..=*max_layer {
        if let Some(depth) = firewall.get(&idx) {
            // packet arrives
            let time = idx;
            // ping-pong movement.
            // it's is rather simple to check for zero-position.
            if time.is_multiple_of(depth * 2 - 2) {
                severity += idx * depth;
            }
        }
    }

    severity
}

fn part2(firewall: &Firewall) -> u32 {
    // What is the fewest number of picoseconds that you need to delay the packet
    // to pass through the firewall without being caught?
    let mut delay = 0;
    let max_layer = firewall.keys().max().unwrap();

    loop {
        let mut found = true;
        for idx in 0..=*max_layer {
            if let Some(depth) = firewall.get(&idx) {
                let delayed = idx + delay;
                if delayed.is_multiple_of(depth * 2 - 2) {
                    found = false;
                    break;
                }
            }
        }
        if found {
            break delay;
        }
        delay += 1;
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day13: Packet Scanners ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
