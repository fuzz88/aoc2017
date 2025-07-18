use std::env;
use std::error;
use std::fs;
use std::ops;

// brilliant tutorial on hexagonal grids.
// https://www.redblobgames.com/grids/hexagons/#coordinates

//      \ n  /
//    nw +--+ ne
//      /    \
//    -+      +-
//      \    /
//    sw +--+ se
//      / s  \

#[derive(Clone, Copy)]
struct HexPoint(i32, i32);
#[derive(Clone, Copy)]
struct HexShift(i32, i32);

impl ops::Add<HexShift> for HexPoint {
    type Output = HexPoint;

    fn add(self, rhs: HexShift) -> HexPoint {
        HexPoint(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl HexPoint {
    fn axial_subtract(self, b: HexPoint) -> HexPoint {
        HexPoint(self.0 - b.0, self.1 - b.1)
    }

    fn axial_distance(self, b: HexPoint) -> u32 {
        let vec = self.axial_subtract(b);
        ((i32::abs(vec.0) + i32::abs(vec.0 + vec.1) + i32::abs(vec.1)) / 2) as u32
    }
}

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

    #[rustfmt::skip]
    fn get_shift(&self) -> HexShift {
        // axial coordinates
        // flat rotation
        match self {
            Dir::N  => HexShift(0, -1),
            Dir::NE => HexShift(1, -1),
            Dir::SE => HexShift(1, 0),
            Dir::S  => HexShift(0, 1),
            Dir::SW => HexShift(-1, 1),
            Dir::NW => HexShift(-1, 0),
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

fn part1(directions: &Vec<Dir>) -> u32 {
    // Starting where the child process started,
    // you need to determine the fewest number of steps required to reach him.
    let end_point = directions.iter().fold(HexPoint(0, 0), |point, direction| {
        point + direction.get_shift()
    });

    HexPoint(0, 0).axial_distance(end_point)
}

fn part2(directions: &Vec<Dir>) -> u32 {
    // How many steps away is the furthest
    // the child process ever got from his starting position?
    let mut max_dist = 0;
    let mut current_point = HexPoint(0, 0);

    for direction in directions {
        let next_point = current_point + direction.get_shift();
        max_dist = max_dist.max(next_point.axial_distance(HexPoint(0, 0)));
        current_point = next_point;
    }

    max_dist
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day11: Hex Ed ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
