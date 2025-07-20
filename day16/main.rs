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

    fn apply(&self, state: VecDeque<u8>) -> VecDeque<u8> {
        let mut state = state.clone();

        match self {
            Move::Spin(x) => {
                (0..*x).for_each(|_| {
                    let el = state.pop_back().unwrap();
                    state.push_front(el);
                });
            }
            Move::Exchange(a, b) => {
                state.swap(*a, *b);
            }
            Move::Partner(a, b) => {
                let a = state.iter().position(|x| x == a).unwrap();
                let b = state.iter().position(|x| x == b).unwrap();
                state.swap(a, b);
            }
        };

        state
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
    let state: VecDeque<u8> = (b'a'..=b'p').collect();

    moves
        .iter()
        .fold(state, |s, m| m.apply(s))
        .iter()
        .map(|c| *c as char)
        .collect()
}

fn part2(moves: &[Move]) -> String {
    let mut state: VecDeque<u8> = (b'a'..=b'p').collect();
    
    // dance is cycled somehow
    (0..100).for_each(|_| {
        state = moves.iter().fold(state.clone(), |s, m| m.apply(s));
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
