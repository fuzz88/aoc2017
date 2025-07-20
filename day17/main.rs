fn part1(steps: usize) -> u32 {
    let mut buf = vec![0];
    let mut idx = 0;

    (1..=2017).for_each(|el| {
        idx = (idx + steps) % buf.len() + 1;
        buf.insert(idx, el);
    });

    buf[idx + 1]
}

fn part2(steps: usize) -> u32 {
    let mut result = 0;
    let mut idx = 0;
    let mut buf_len = 1;

    (1..=50_000_000).for_each(|el| {
        idx = (idx + steps) % buf_len + 1;
        if idx == 1 {
            result = el;
        }
        buf_len += 1;
    });

    result
}

fn main() {
    println!("--- Day17: Spinlock ---");

    let input_data = 370;

    println!("{}", part1(input_data));
    println!("{}", part2(input_data));
}
