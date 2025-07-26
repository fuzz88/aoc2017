use std::collections::HashSet;
use std::env;
use std::error;
use std::fs;

type Nodes = HashSet<(i64, i64)>;

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn read_input(filename: &str) -> Result<Nodes, Box<dyn error::Error>> {
    let map = fs::read_to_string(filename)?.as_bytes().to_owned();

    // side of the square without newlines.
    let n = usize::isqrt(map.len());
    let d = n / 2;

    let mut nodes = HashSet::new();

    for i in 0..n {
        for j in 0..n {
            if map[i * (n + 1) + j] == 35 {
                let x = j as i64 - d as i64;
                let y = i as i64 - d as i64;
                nodes.insert((x, y));
            }
        }
    }

    Ok(nodes)
}

fn next_coordinates(coordinates: (i64, i64), direction: &Direction) -> (i64, i64) {
    match direction {
        Direction::Up => (coordinates.0, coordinates.1 - 1),
        Direction::Right => (coordinates.0 + 1, coordinates.1),
        Direction::Down => (coordinates.0, coordinates.1 + 1),
        Direction::Left => (coordinates.0 - 1, coordinates.1),
    }
}

fn next_direction_simple(direction: &Direction, infected: bool) -> Direction {
    match (direction, infected) {
        (Direction::Up, true) => Direction::Right,
        (Direction::Up, false) => Direction::Left,
        (Direction::Right, true) => Direction::Down,
        (Direction::Right, false) => Direction::Up,
        (Direction::Down, true) => Direction::Left,
        (Direction::Down, false) => Direction::Right,
        (Direction::Left, true) => Direction::Up,
        (Direction::Left, false) => Direction::Down,
    }
}

fn part1(nodes: &Nodes) -> usize {
    // Given your actual map, after 10000 bursts of activity,
    // how many bursts cause a node to become infected?
    // (Do not count nodes that begin infected.)

    let mut infected_nodes = nodes.clone();

    let mut current_direction = Direction::Up;
    let mut current_coordinates = (0, 0);

    let mut burst_count = 0;
    let mut infections = 0;

    loop {
        if infected_nodes.contains(&current_coordinates) {
            current_direction = next_direction_simple(&current_direction, true);
            infected_nodes.remove(&current_coordinates);
        } else {
            current_direction = next_direction_simple(&current_direction, false);
            infected_nodes.insert(current_coordinates);
            infections += 1;
        }

        current_coordinates = next_coordinates(current_coordinates, &current_direction);

        burst_count += 1;

        if burst_count == 10000 {
            break;
        }
    }

    infections
}

fn next_direction_complex(direction: &Direction, status: &Status) -> Direction {
    match (direction, status) {
        (Direction::Up, Status::Clean) => Direction::Left,
        (Direction::Up, Status::Weakened) => Direction::Up,
        (Direction::Up, Status::Infected) => Direction::Right,
        (Direction::Up, Status::Flagged) => Direction::Down,

        (Direction::Down, Status::Clean) => Direction::Right,
        (Direction::Down, Status::Weakened) => Direction::Down,
        (Direction::Down, Status::Infected) => Direction::Left,
        (Direction::Down, Status::Flagged) => Direction::Up,

        (Direction::Left, Status::Clean) => Direction::Down,
        (Direction::Left, Status::Weakened) => Direction::Left,
        (Direction::Left, Status::Infected) => Direction::Up,
        (Direction::Left, Status::Flagged) => Direction::Right,

        (Direction::Right, Status::Clean) => Direction::Up,
        (Direction::Right, Status::Weakened) => Direction::Right,
        (Direction::Right, Status::Infected) => Direction::Down,
        (Direction::Right, Status::Flagged) => Direction::Left,
    }
}

fn part2(nodes: &Nodes) -> usize {
    // Given your actual map, after 10000000 bursts of activity,
    // how many bursts cause a node to become infected?
    // (Do not count nodes that begin infected.)

    let mut infected_nodes = nodes.clone();
    let mut weakened_nodes = HashSet::new();
    let mut flagged_nodes = HashSet::new();

    let mut current_direction = Direction::Up;
    let mut current_coordinates = (0, 0);

    let mut burst_count = 0;
    let mut infections = 0;

    loop {
        if infected_nodes.contains(&current_coordinates) {
            current_direction = next_direction_complex(&current_direction, &Status::Infected);
            infected_nodes.remove(&current_coordinates);
            flagged_nodes.insert(current_coordinates);
        } else if weakened_nodes.contains(&current_coordinates) {
            current_direction = next_direction_complex(&current_direction, &Status::Weakened);
            weakened_nodes.remove(&current_coordinates);
            infected_nodes.insert(current_coordinates);
            infections += 1;
        } else if flagged_nodes.contains(&current_coordinates) {
            current_direction = next_direction_complex(&current_direction, &Status::Flagged);
            flagged_nodes.remove(&current_coordinates);
        } else {
            current_direction = next_direction_complex(&current_direction, &Status::Clean);
            weakened_nodes.insert(current_coordinates);
        }

        current_coordinates = next_coordinates(current_coordinates, &current_direction);

        burst_count += 1;

        if burst_count == 10_000_000 {
            break;
        }
    }

    infections
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day22: Sporifica Virus ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
