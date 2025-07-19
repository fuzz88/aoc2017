use std::env;
use std::error;
use std::fs;

fn read_input(filename: &str) -> Result<String, Box<dyn error::Error>> {
    let stream = fs::read_to_string(filename)?;
    Ok(stream)
}

fn count_score_and_garbage(stream: &str) -> (u32, u32) {
    let stream = stream.as_bytes();
    let mut idx = 0;

    let mut score = 0;
    let mut weight = 0;

    let mut garbage = 0;

    loop {
        // println!("{weight}");

        if idx == stream.len() {
            break;
        }

        match stream[idx] {
            b'{' => {
                weight += 1;
                idx += 1;
                continue;
            }
            b'}' => {
                score += weight;
                weight -= 1;
                idx += 1;
                continue;
            }
            b'!' => {
                idx += 2;
                continue;
            }
            b'<' => {
                idx += 1;
                while stream[idx] != b'>' {
                    if stream[idx] == b'!' {
                        idx += 2;
                    } else {
                        garbage += 1;
                        idx += 1;
                    }
                }
                idx += 1;
                continue;
            }
            _ => {
                idx += 1;
            }
        };
    }

    (score, garbage)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day9: Stream Processing ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    let (part1, part2) = count_score_and_garbage(&input_data);

    println!("{part1}\n{part2}");

    Ok(())
}
