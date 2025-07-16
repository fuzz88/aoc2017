use std::env;
use std::error;
use std::fs;

fn read_input(filename: &str) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let lengths = fs::read_to_string(filename)?
        .split(',')
        .map(|l| l.trim_end().parse().unwrap())
        // .inspect(|l| println!("{}", l))
        .collect();

    Ok(lengths)
}

fn reverse(list: &mut Vec<u8>, position: usize, length: usize) {
    // println!("{} {}", position, length);
    // println!("{:?}", list);

    let end_pos = position + length - 1;

    let mut i = position % list.len();
    let mut j = end_pos % list.len();
    while i != (position + length / 2) % list.len() {
        // println!("{} {}", i, j);
        if i != j {
            let t = list[i];
            list[i] = list[j];
            list[j] = t;
        }
        i += 1;
        i = i % list.len();
        if j == 0 {
            j = list.len();
        }
        j -= 1;
    }

    // println!("{:?}", list);
}

fn part1(input: &Vec<usize>) -> u16 {
    let mut list = vec![];
    (0..=255).for_each(|num| list.push(num as u8));

    let mut skip = 0;
    let mut position = 0;

    for length in input {
        reverse(&mut list, position, *length);
        position += (*length + skip) % list.len();
        skip += 1;
    }

    list[0] as u16 * list[1] as u16
}

fn part2() -> String {
    let lengths = include_bytes!("input.txt");
    let mut lengths: Vec<usize> = lengths[0..lengths.len() - 1]
        .iter()
        .copied()
        .map(|v| v as usize)
        .collect();

    lengths.extend([17, 31, 73, 47, 23]);
    // println!("{:?}", lengths);

    let mut sparse = vec![];
    (0..=255).for_each(|num| sparse.push(num as u8));

    let mut position = 0;
    let mut skip = 0;

    (0..64).for_each(|_| {
        for length in &lengths {
            reverse(&mut sparse, position, *length);
            position += (*length + skip) % sparse.len();
            skip += 1;
        }
    });

    let dense: Vec<u8> = sparse
        .chunks(16)
        .map(|chunk| chunk.iter().copied().reduce(|acc, e| acc ^ e).unwrap())
        .collect();

    // println!("{:?}", list);

    let hash = dense
        .iter()
        .map(|num| format!("{num:02x}"))
        .collect::<Vec<_>>()
        .join("");

    // println!("{hash}");
    hash
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day10: Knot Hash ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2());

    Ok(())
}
