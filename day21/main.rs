use std::env;
use std::error;
use std::fs;
use std::mem;

type Rule = (Vec<Vec<u8>>, Vec<u8>);

fn read_input(filename: &str) -> Result<Vec<Rule>, Box<dyn error::Error>> {
    let rules = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.split(" => ").map(|s| s.as_bytes().to_owned()))
        .map(|mut bytes| (vec![bytes.next().unwrap()], bytes.next().unwrap()))
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

fn flip_vertically(image: &mut [u8], n: usize) {
    for i in 0..n / 2 {
        for j in 0..n {
            let idx_a = i * (n + 1) + j;
            let idx_b = (n + 1) * (n - i - 1) + j;
            mem::swap(
                unsafe { image.as_mut_ptr().add(idx_a).as_mut().unwrap() },
                unsafe { image.as_mut_ptr().add(idx_b).as_mut().unwrap() },
            );
        }
    }
}

fn flip_horizontally(image: &mut [u8], n: usize) {
    for i in 0..n {
        for j in 0..n / 2 {
            let idx_a = i * (n + 1) + j;
            let idx_b = i * (n + 1) + (n - j - 1);
            mem::swap(
                unsafe { image.as_mut_ptr().add(idx_a).as_mut().unwrap() },
                unsafe { image.as_mut_ptr().add(idx_b).as_mut().unwrap() },
            );
        }
    }
}

fn transpose(image: &mut [u8], n: usize) {
    for i in 0..n {
        for j in i..n {
            let idx_a = i * (n + 1) + j;
            let idx_b = j * (n + 1) + i;
            mem::swap(
                unsafe { image.as_mut_ptr().add(idx_a).as_mut().unwrap() },
                unsafe { image.as_mut_ptr().add(idx_b).as_mut().unwrap() },
            );
        }
    }
}

fn rotate_ccw(image: &mut [u8], n: usize, count: usize) {
    for _ in 0..count {
        transpose(image, n);
        flip_vertically(image, n);
    }
}

fn is_rule_matched(image: &[u8], rule: &Rule) -> bool {
    let patterns = &rule.0;
    patterns.iter().any(|pattern| pattern == image)
}

fn augment_rules(rules: &mut [Rule]) {
    for rule in rules {
        let n = usize::isqrt(rule.0[0].len());

        let mut augmented = rule.0[0].clone();
        flip_vertically(&mut augmented, n);
        rule.0.push(augmented.clone());

        (1..=3).for_each(|count| {
            rotate_ccw(&mut augmented, n, count);
            rule.0.push(augmented.clone());
        });

        let mut augmented = rule.0[0].clone();
        flip_horizontally(&mut augmented, n);
        rule.0.push(augmented.clone());

        (1..=3).for_each(|count| {
            rotate_ccw(&mut augmented, n, count);
            rule.0.push(augmented.clone());
        });

        let mut augmented = rule.0[0].clone();

        (1..=3).for_each(|count| {
            rotate_ccw(&mut augmented, n, count);
            rule.0.push(augmented.clone());
        });

        rule.0.sort();
        rule.0.dedup();
    }
}

fn part1(rules: &[Rule]) -> usize {
    let mut rules = rules.to_owned();
    augment_rules(&mut rules);

    //     .#.
    //     ..#
    //     ###

    let mut image = vec![46, 35, 46, 47, 46, 46, 35, 47, 35, 35, 35];

    for rule in rules {
        if is_rule_matched(&image, &rule) {
            println!("matched");
            print_image(&rule.0[0]);
            println!();
            print_image(&rule.1);
            image = rule.1.clone();
        }
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
