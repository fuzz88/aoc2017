use std::env;
use std::error;
use std::fs;

//      \ n  /
//    nw +--+ ne
//      /    \
//    -+      +-
//      \    /
//    sw +--+ se
//      / s  \

#[derive(Debug)]
enum Dir {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl Dir {
    #[rustfmt::skip]
    fn from_str(direction: &str) -> Self {
        match direction {
            "n"  => Dir::N,
            "ne" => Dir::NE,
            "se" => Dir::SE,
            "s"  => Dir::S,
            "sw" => Dir::SW,
            "nw" => Dir::NW,
            dir  => unimplemented!("unknown direction: {}", dir),
        }
    }
}

fn read_input(filename: &str) -> Result<Vec<Dir>, Box<dyn error::Error>> {
    let directions = fs::read_to_string(filename)?
        .split(',')
        .map(|direction| Dir::from_str(direction.trim_end()))
        .collect();

    Ok(directions)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day11: Hex Ed ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{:?}", input_data);

    Ok(())
}
