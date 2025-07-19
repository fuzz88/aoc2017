use std::env;
use std::error;
use std::fs;

#[derive(Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl Move {
    fn from_str(s: &str) -> Self {
        let sb = s.as_bytes();
        match &sb[0] {
            b's' => Move::Spin(s.split_at(1).1.parse().unwrap()),
            b'x' => {
                let programs: Vec<usize> = s
                    .split_at(1)
                    .1
                    .split("/")
                    .map(|el| el.parse().unwrap())
                    .collect();
                Move::Exchange(programs[0], programs[1])
            }
            b'p' => Move::Partner(sb[1], sb[3]),
            mv => unimplemented!("unknown move: {}", mv),
        }
    }
}

type Moves = Vec<Move>;

fn read_input(filename: &str) -> Result<Moves, Box<dyn error::Error>> {
    let moves = fs::read_to_string(filename)?
        .trim_end()
        .split(',')
        .map(Move::from_str)
        .collect();

    Ok(moves)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day16: Permutation Promenade ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{:?}", &input_data);

    Ok(())
}
