use std::collections::VecDeque;
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

    fn apply_mut(&self, state: &mut VecDeque<u8>) {
        match self {
            Move::Spin(x) => {
                state.rotate_right(*x);
            }
            Move::Exchange(a, b) => {
                state.swap(*a, *b);
            }
            Move::Partner(a, b) => {
                let mut idx_a = 0;
                let mut idx_b = 0;
                let mut count = 0;

                for (idx, x) in state.iter().enumerate() {
                    if x == a {
                        idx_a = idx;
                        count += 1;
                    }
                    if x == b {
                        idx_b = idx;
                        count += 1;
                    }
                    if count == 2 {
                        break;
                    }
                }

                state.swap(idx_a, idx_b);
            }
        };
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

fn part1(moves: &[Move]) -> String {
    let mut state: VecDeque<u8> = (b'a'..=b'p').collect();

    moves.iter().for_each(|m| {
        m.apply_mut(&mut state);
    });

    state.iter().map(|c| *c as char).collect()
}

fn part2(moves: &[Move]) -> String {
    let mut state: VecDeque<u8> = (b'a'..=b'p').collect();

    // dance is cycled somehow
    // no need to do 1 billion iterations.
    (0..100).for_each(|_| {
        moves.iter().for_each(|m| {
            m.apply_mut(&mut state);
        });
    });

    state.iter().map(|c| *c as char).collect()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day16: Permutation Promenade ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
