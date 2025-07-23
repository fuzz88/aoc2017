use std::env;
use std::error;
use std::fs;

type Map = Vec<u8>;

fn read_input(filename: &str) -> Result<Map, Box<dyn error::Error>> {
    let map = fs::read_to_string(filename)?.as_bytes().to_vec();

    Ok(map)
}

fn walker<F>(map: &Map, mut visit: F)
where
    F: FnMut(u8),
{
    let line_count = map.iter().fold(0, |acc, el| acc + (*el == 10) as usize);
    let line_length = map.len() / line_count;

    let mut idx = 0;
    let starting_point = loop {
        if map[idx] == b'|' {
            break idx;
        }
        idx += 1;
    };

    // starting from first `|`, going down.
    let mut current_point = starting_point as isize;
    let mut dx: isize = 0;
    let mut dy: isize = 1;

    'walk: loop {
        // walking till `+` or stop walking if reached ` `
        while map[current_point as usize] != b'+' {
            visit(map[current_point as usize]);

            if map[current_point as usize] == b' ' {
                break 'walk;
            }
            current_point += dy * line_length as isize;
        }

        // standing on `+`
        visit(map[current_point as usize]);

        // change direction, left or right
        dy = 0;
        if current_point > 0 && map[current_point as usize - 1] != b' ' {
            dx = -1;
        }
        if current_point as usize + 1 < map.len() && map[current_point as usize + 1] != b' ' {
            dx = 1;
        }
        // step in new direction
        current_point += dx;

        // walking till `+` or stop walking if reached ` `
        while map[current_point as usize] != b'+' {
            visit(map[current_point as usize]);

            if map[current_point as usize] == b' ' {
                break 'walk;
            }
            current_point += dx;
        }
        // standing on `+`
        visit(map[current_point as usize]);

        // change direction, up or down
        dx = 0;
        if current_point - line_length as isize >= 0
            && map[current_point as usize - line_length] != b' '
        {
            dy = -1;
        }
        if current_point as usize + line_length < map.len()
            && map[current_point as usize + line_length] != b' '
        {
            dy = 1;
        }
        // step in new direction
        current_point += dy * line_length as isize;

        // repeat
    }
}

fn part1(map: &Map) -> String {
    // What letters will it see (in the order it would see them)
    // if it follows the path?
    let mut letters = String::new();

    walker(map, |cell| {
        if cell != b'|' && cell != b'-' && cell != b'+' && cell != b' ' {
            letters.push(cell as char);
        }
    });

    letters
}

fn part2(map: &Map) -> usize {
    // How many steps does the packet need to go?
    let mut steps = 0;

    walker(map, |_| {
        steps += 1;
    });
    
    // don't count standing at start as a step.
    steps - 1
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day19: A Series of Tubes ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
