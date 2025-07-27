use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "left." => Direction::Left,
            "right." => Direction::Right,
            direction => unimplemented!("unknown direction: {}", direction),
        }
    }
}

#[derive(Debug)]
struct Action {
    write_value: u8,
    next_direction: Direction,
    next_state: String,
}

#[derive(Debug)]
struct State {
    on_zero: Action,
    on_one: Action,
}

type Tape = HashMap<i64, u8>;
type States = HashMap<String, State>;

type Blueprint = (String, States, usize);

fn read_input(filename: &str) -> Result<Blueprint, Box<dyn error::Error>> {
    let content = fs::read_to_string(filename)?;
    let mut lines = content.lines();

    let start_state = &lines.next().unwrap().split_whitespace().last().unwrap()[0..1];
    let steps = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut states = States::new();

    loop {
        if lines.next() == None {
            break;
        }

        let state_name = &lines.next().unwrap().split_whitespace().last().unwrap()[0..1];
        let _zero_if = lines.next();
        let write_if_0 = lines.next().unwrap().split_whitespace().last().unwrap()[0..1]
            .parse::<u8>()
            .unwrap();
        let dir_if_0 = Direction::from(lines.next().unwrap().split_whitespace().last().unwrap());
        let state_if_0 = &lines.next().unwrap().split_whitespace().last().unwrap()[0..1];

        let _one_if = lines.next();
        let write_if_1 = lines.next().unwrap().split_whitespace().last().unwrap()[0..1]
            .parse::<u8>()
            .unwrap();
        let dir_if_1 = Direction::from(lines.next().unwrap().split_whitespace().last().unwrap());
        let state_if_1 = &lines.next().unwrap().split_whitespace().last().unwrap()[0..1];

        let action_0 = Action {
            write_value: write_if_0,
            next_direction: dir_if_0,
            next_state: state_if_0.to_string(),
        };

        let action_1 = Action {
            write_value: write_if_1,
            next_direction: dir_if_1,
            next_state: state_if_1.to_string(),
        };

        let state = State {
            on_zero: action_0,
            on_one: action_1,
        };

        states.insert(state_name.to_string(), state);
    }

    let blueprint = (start_state.to_string(), states, steps);

    Ok(blueprint)
}

fn part1(blueprint: &Blueprint) -> usize {
    // What is the diagnostic checksum it produces once it's working again?

    let mut current_step = 0;
    let mut tape = Tape::new();
    let mut tape_pos = 0;

    let mut current_state = &blueprint.0;
    let states = &blueprint.1;

    loop {
        let state = states.get(current_state).unwrap();

        let tape_entry = tape.entry(tape_pos).or_insert(0);

        if *tape_entry == 0 {
            let value = state.on_zero.write_value;
            *tape_entry = value;

            let direction = &state.on_zero.next_direction;
            tape_pos += match direction {
                Direction::Left => -1,
                Direction::Right => 1,
            };
            current_state = &state.on_zero.next_state;
        } else {
            let value = state.on_one.write_value;
            *tape_entry = value;

            let direction = &state.on_one.next_direction;
            tape_pos += match direction {
                Direction::Left => -1,
                Direction::Right => 1,
            };
            current_state = &state.on_one.next_state;
        }

        current_step += 1;

        if current_step == blueprint.2 {
            break;
        }
    }

    tape.values().map(|&v| v as usize).sum()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day25: The Halting Problem ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));

    Ok(())
}
