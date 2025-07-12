use std::error;

fn part1(addr: u32) -> u32 {
    if addr == 1 {
        return 0;
    }

    let mut q = 1;
    let k;

    let side = loop {
        // previous powers could be memorized, but still ok for the task.
        // compiler must get powers out of the loop as invariants though.
        if u32::pow(q, 2) >= addr {
            // println!("square = {}", q);
            k = (addr - u32::pow(q - 2, 2) + 1) % (u32::pow(q, 2) - u32::pow(q - 2, 2));
            // println!("idx on square = {}", k);
            break k / q;
        }
        q += 2
    };

    // println!("side = {}", side);
    let shift = (k - (q-1) * side - (q - 1) / 2 - 1) as i32;
    // println!("shift = {}", shift);
    let walls = (q - 1) / 2;

    i32::abs(shift) as u32 + walls 
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 3: Spiral Memory ---");

    let input_data = 325489;

    println!("{}", part1(input_data));

    Ok(())
}
