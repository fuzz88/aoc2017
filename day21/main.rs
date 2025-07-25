use std::env;
use std::error;
use std::fs;
use std::mem;

type Rule = (Vec<u8>, Vec<u8>);

fn read_input(filename: &str) -> Result<Vec<Rule>, Box<dyn error::Error>> {
    let rules = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.split(" => ").map(|s| s.as_bytes().to_owned()))
        .map(|mut bytes| (bytes.next().unwrap(), bytes.next().unwrap()))
        .collect();

    Ok(rules)
}

fn print_image(image: &[u8]) {
    image.iter().for_each(|pixel| match pixel {
        b'/' => {
            print!("\n");
        }
        sym => {
            print!("{}", *sym as char);
        }
    });
    print!("\n");
}

fn flip_vertically(image: &[u8]) -> Vec<u8> {
    image
        .split(|p| *p == 47)
        .rev()
        .collect::<Vec<_>>()
        .join(&47)
        .to_owned()
}

fn flip_horizontally(image: &[u8]) -> Vec<u8> {
    image
        .split(|p| *p == 47)
        .map(|row| row.iter().copied().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .join(&47)
        .to_owned()
}

fn transpose(image: &[u8]) -> Vec<u8> {
    let n = usize::isqrt(image.len());

    let mut result = image
        .split(|p| *p == 47)
        .map(|row| row.iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..n {
        for j in i..n {
            let t = result[i][j];
            result[i][j] = result[j][i];
            result[j][i] = t;
        }
    }

    result.join(&47).iter().copied().collect::<Vec<_>>()
}

fn rotate_ccw(image: &[u8]) -> Vec<u8> {
    flip_vertically(&transpose(image))
}

fn match_rule(image: &[u8], rule: &Rule) -> usize {
    print_image(image);
    println!();
    print_image(&rule.0);
    println!();
    print_image(&flip_vertically(&rule.0));
    println!();
    print_image(&flip_horizontally(&rule.0));
    println!();
    print_image(&rotate_ccw(&rule.0));
    println!();
    print_image(&rotate_ccw(&rotate_ccw(&rule.0)));
    println!();
    print_image(&rotate_ccw(&rotate_ccw(&rotate_ccw(&rule.0))));

    if image == rule.0 {
        1
    } else {
        0
    }
}

fn part1(rules: &[Rule]) -> usize {
    println!("{:?}", rules);

    let glider = [46, 35, 46, 47, 46, 46, 35, 47, 35, 35, 35];
    for rule in rules {
        println!("------------");
        println!("{}", match_rule(&glider, rule));
        println!("------------");
    }
    0
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day21: Fractal Art ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));

    Ok(())
}
