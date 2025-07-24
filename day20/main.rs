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

impl Particle {
    fn update(&mut self) {
        self.velocity[0] += self.acceleration[0];
        self.position[0] += self.velocity[0];
        self.velocity[1] += self.acceleration[1];
        self.position[1] += self.velocity[1];
        self.velocity[2] += self.acceleration[2];
        self.position[2] += self.velocity[2];
    }
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

    // first we need to find particles with the lowest acceleration,
    // because in the long term these particles will scatter less.
    // if the lowest accelerations are equal for some group of particles,
    // we must look for particles with the lowest velocity within this group.
    // if after that we have group of particles moving equally slow,
    // or not moving at all,
    // closest to <0, 0, 0> will be the one which is already closest.

    // sort by acceleration, velocity and position in this particular order,
    // get the minimum.

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

fn filter_out_collided(particles: &mut Vec<Particle>) -> usize {
    let mut removed_count = 0;

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
            removed_count += 1;
        } else {
            if removing {
                particles.remove(c - 1);
                removed_count += 1;
                removing = false;
            } else {
                c += 1;
            }
        }
    }

    removed_count
}

fn calculate_pairs_distances(particles: &Vec<Particle>) -> Vec<i64> {
    let mut distances = vec![];

    for (idx1, p1) in particles.iter().enumerate() {
        for (idx2, p2) in particles.iter().enumerate() {
            if idx1 != idx2 {
                distances.push(
                    i64::abs(p1.position[0] - p2.position[0])
                        + i64::abs(p1.position[1] - p2.position[1])
                        + i64::abs(p1.position[2] - p2.position[2]),
                );
            }
        }
    }

    distances
}

fn part2(particles: &Vec<Particle>) -> usize {
    // How many particles are left after all collisions are resolved?

    let mut particles = particles.clone();

    loop {
        let distances = calculate_pairs_distances(&particles);

        particles.iter_mut().for_each(|p| {
            p.update();
        });

        let removed_count = filter_out_collided(&mut particles);

        if removed_count == 0 {
            let next_distances = calculate_pairs_distances(&particles);
            if distances
                .iter()
                .zip(next_distances.iter())
                .map(|(a, b)| a - b)
                .all(|d| d < 0)
            {
                // all collisions are resolved when particles
                // are scattering away from each other.
                break;
            }
        }
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
