use std::error;

fn part1(addr: u32) -> u32 {
    if addr == 1 {
        return 0;
    }

    let mut q = 1;
    let k;

    let side = loop {
        // previous powers could be memorized, but still ok for the task.
        // compiler should get powers out of the loop as invariants.
        if u32::pow(q, 2) >= addr {
            // q^2 is the amount of elements the square contains.
            // first q when q^2 >= addr is the `q x q` square on the side of which we have `addr`
            // element.
            //
            // println!("square = {}", q);
            //
            // if we count elements on square's sides starting from right lower corner,
            // what is the index of `addr` element in that sequence?
            k = (addr - u32::pow(q - 2, 2) + 1) % (u32::pow(q, 2) - u32::pow(q - 2, 2));
            // println!("idx on square = {}", k);
            //
            // if we devide this index by the size of the side of the square,
            // we get the index of the side which contains `addr` element.
            break k / q;
        }
        q += 2
    };

    // println!("side = {}", side);
    //
    // how much `addr` element shifted from central element of the side?
    // this is our first coordinate.
    let shift = (k - (q - 1) * side - (q - 1) / 2 - 1) as i32;
    // println!("shift = {}", shift);
    //
    // how much central element of the side is distant from (0, 0)?
    // this is our second coordinate,
    // always positive number, sign doesn't matter,
    // since point to calculate manhattan distance from is (0, 0)
    let walls = (q - 1) / 2;

    // manhattan distance
    i32::abs(shift) as u32 + walls
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 3: Spiral Memory ---");

    let input_data = 325489;

    println!("{}", part1(input_data));

    Ok(())
}
