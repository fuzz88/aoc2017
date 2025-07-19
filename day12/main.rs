use std::collections::{HashMap, VecDeque};
use std::env;
use std::error;
use std::fs;

type Graph = HashMap<u32, Vec<u32>>;

fn parse_line(line: &str) -> (u32, Vec<u32>) {
    let mut components = line
        .split_whitespace()
        .map(|node| node.strip_suffix(",").unwrap_or(node));

    let node = components.next().unwrap().parse().unwrap();
    let connected = components.skip(1).map(|num| num.parse().unwrap()).collect();

    (node, connected)
}

fn read_input(filename: &str) -> Result<Graph, Box<dyn error::Error>> {
    let graph = fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect();

    Ok(graph)
}

fn bfs<F>(graph: &Graph, start: u32, mut visit: F)
where
    F: FnMut(u32),
{
    let mut visited = vec![];
    let mut to_process = VecDeque::new();

    to_process.push_back(start);
    visited.push(start);

    while let Some(node) = to_process.pop_front() {
        visit(node);

        if let Some(neighbours) = graph.get(&node) {
            for neighbour in neighbours {
                // is it faster than hashing u32 with HashSet?
                match visited.binary_search(neighbour) {
                    Ok(_) => {
                        continue;
                    }
                    Err(idx) => {
                        // inserting at `idx` keeps `visited` sorted.
                        visited.insert(idx, *neighbour);
                        to_process.push_back(*neighbour);
                    }
                };
            }
        }
    }
}

fn part1(graph: &Graph) -> u32 {
    // How many programs are in the group that contains program ID 0?
    let mut group_size = 0;

    bfs(graph, 0, |_| group_size += 1);

    group_size
}

fn part2(graph: &Graph) -> u32 {
    // How many groups are there in total?
    let mut group_count = 0;

    let mut nodes: Vec<u32> = graph.keys().copied().collect();
    nodes.sort();

    while let Some(start) = nodes.pop() {
        group_count += 1;

        bfs(graph, start, |node| {
            if let Ok(index) = nodes.binary_search(&node) {
                // removing from sorted vec keeps it sorted.
                nodes.remove(index);
            }
        });
    }

    group_count
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day12: Digital Plumber ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{}", part1(&input_data));
    println!("{}", part2(&input_data));

    Ok(())
}
