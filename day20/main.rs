use std::env;
use std::error;
use std::fs;

#[rustfmt::skip]
#[derive(Debug)]
struct Particle {
    position:       [i32; 3],
    velocity:       [i32; 3],
    acceleration:   [i32; 3],
}

impl From<&str> for Particle {
    fn from(value: &str) -> Self {
        let mut components = value.split(", ");

        let positions = components.next().unwrap();
        let mut positions = positions[3..positions.len() - 1].split(",");
        let position = [
            positions.next().unwrap().parse().unwrap(),
            positions.next().unwrap().parse().unwrap(),
            positions.next().unwrap().parse().unwrap(),
        ];

        let velocities = components.next().unwrap();
        let mut velocities = velocities[3..velocities.len() - 1].split(",");
        let velocity = [
            velocities.next().unwrap().parse().unwrap(),
            velocities.next().unwrap().parse().unwrap(),
            velocities.next().unwrap().parse().unwrap(),
        ];

        let accelerations = components.next().unwrap();
        let mut accelerations = accelerations[3..accelerations.len() - 1].split(",");
        let acceleration = [
            accelerations.next().unwrap().parse().unwrap(),
            accelerations.next().unwrap().parse().unwrap(),
            accelerations.next().unwrap().parse().unwrap(),
        ];

        Particle {
            position,
            velocity,
            acceleration,
        }
    }
}

fn read_input(filename: &str) -> Result<Vec<Particle>, Box<dyn error::Error>> {
    let particles = fs::read_to_string(filename)?
        .lines()
        .map(Particle::from)
        .collect();

    Ok(particles)
}

fn part1(particles: &[Particle]) -> usize {
    // Which particle will stay closest to position <0,0,0> in the long term?

    let min_accel = particles
        .iter()
        .enumerate()
        .map(|(idx, particle)| {
            (
                particle
                    .acceleration
                    .iter()
                    .copied()
                    .fold(0, |acc, c| acc + i32::abs(c)),
                idx,
            )
        })
        .min()
        .unwrap();

    min_accel.1
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day20: Particle Swarm ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));

    Ok(())
}
