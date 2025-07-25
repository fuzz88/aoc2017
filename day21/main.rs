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

fn match_rule(image: &[u8], rule: &Rule) -> usize {
    let n = usize::isqrt(image.len());

    print_image(&image);
    println!("flipping horizontally:");
    let mut image1 = image.to_owned();
    flip_horizontally(&mut image1, n);
    print_image(&image1);
    println!("flipping vertically:");
    let mut image2 = image.to_owned();
    flip_vertically(&mut image2, n);
    print_image(&image2);
    println!("transposing:");
    let mut image3 = image.to_owned();
    transpose(&mut image3, n);
    print_image(&image3);
    println!("rotating ccw:");
    let mut image4 = image.to_owned();
    rotate_ccw(&mut image4, n, 1);
    print_image(&image4);
    // print_image(&rotate_ccw(&rule.0));
    // println!();
    // print_image(&rotate_ccw(&rotate_ccw(&rule.0)));
    // println!();
    // print_image(&rotate_ccw(&rotate_ccw(&rotate_ccw(&rule.0))));

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
