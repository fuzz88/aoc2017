use std::env;
use std::error;
use std::fs;

#[rustfmt::skip]
#[derive(Debug, Clone)]
struct Particle {
    position:       [i64; 3],
    velocity:       [i64; 3],
    acceleration:   [i64; 3],
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

    // i guess that "in the long term" means "infinity", so
    // first we need to find particles with the lowest acceleration,
    // because in the long term these particles will scatter less.
    // but if accelerations are equal, we must look for particles with the lowest velocity.
    // and if, for example, we have particles moving equally slow,
    // or if they don't move at all,
    // closest to <0, 0, 0> will stay the one which is already closest.

    particles
        .iter()
        .enumerate()
        .map(|(idx, particle)| {
            (
                particle
                    .acceleration
                    .iter()
                    .copied()
                    .fold(0, |acc, c| acc + i64::abs(c)),
                particle
                    .velocity
                    .iter()
                    .copied()
                    .fold(0, |acc, c| acc + i64::abs(c)),
                particle
                    .position
                    .iter()
                    .copied()
                    .fold(0, |acc, c| acc + i64::abs(c)),
                idx,
            )
        })
        // compares tuples lexicographically.
        .min()
        .unwrap()
        .3
}

fn filter_collided(particles: &mut Vec<Particle>) -> &Vec<Particle> {

    particles.sort_by_key(|el| el.position);

    let mut c = 1;
    let mut removing = false;

    loop {
        if c == particles.len() {
            break;
        }
        if particles[c - 1].position == particles[c].position {
            removing = true;
            particles.remove(c - 1);
        } else {
            if removing {
                particles.remove(c - 1);
                removing = false;
            } else {
                c += 1;
            }
        }
    }

    particles
}

fn part2(particles: &Vec<Particle>) -> usize {
    // How many particles are left after all collisions are resolved?

    // idk how to check if all collisions are resolved.
    // just checking 10k ticks.
    let mut n = 10_000;
    let mut particles = particles.clone();

    while n != 0 {
        particles = filter_collided(&mut particles).to_vec();

        particles = particles
            .iter()
            .map(|p| {
                let mut p = p.clone();
                p.velocity[0] += p.acceleration[0];
                p.position[0] += p.velocity[0];
                p.velocity[1] += p.acceleration[1];
                p.position[1] += p.velocity[1];
                p.velocity[2] += p.acceleration[2];
                p.position[2] += p.velocity[2];
                p
            })
            .collect();

        n -= 1;
    }

    particles.len()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day20: Particle Swarm ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
