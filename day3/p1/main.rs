use std::collections::HashMap;
use std::error;

fn part1(addr: u32) -> u32 {
    if addr == 1 {
        return 0;
    }

    let mut q = 3;
    let k;
    let mut square_area = u32::pow(q, 2);
    let mut previous_square_area = 1;

    let side = loop {
        if square_area >= addr {
            // q^2 is the amount of elements the square contains.
            // first q when q^2 >= addr is the `q x q` square on the side of which we have `addr`
            // element.
            //
            // println!("square = {}", q);
            //
            // if we count elements on square's sides starting from the right lower corner,
            // what is the index of `addr` element in that sequence?
            k = (addr - previous_square_area + 1) % (square_area - previous_square_area);
            // println!("idx on square = {}", k);
            //
            // if we devide this index by the size of the side of the square,
            // we get the index of the side which contains `addr` element.
            break k / q;
        }
        q += 2;
        previous_square_area = square_area;
        square_area = u32::pow(q, 2);
    };

    // println!("side = {}", side);
    //
    // how much `addr` element shifted from the central element of the side?
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

type Coordinates = [i32; 2]; // each value has coordinates. center of coordinates is (0, 0)
type Value = u32;

fn get_neighbours(point: Coordinates, spiral: &HashMap<Coordinates, Value>) -> u32 {
    // println!("{:?}", point);
    let mut result = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if !(dx == 0 && dy == 0) {
                let neighbour = [point[0] + dx, point[1] + dy];
                // println!("{:?}", neighbour);
                result += *spiral.get(&neighbour).unwrap_or(&0);
            }
        }
    }
    result
}

fn part2(limit: u32) -> u32 {
    let mut spiral = HashMap::<Coordinates, Value>::new();

    let mut value = 1;
    let mut next_coords = [0, 0];

    // add center of the spiral
    spiral.insert(next_coords, value);

    // let's write loops to add values to spiral in the proper order
    let mut q = 3; // q x q squares for q=3,5,7,9...

    macro_rules! next_check_limit {
        ($dx:literal, $dy:literal) => {
            next_coords = [next_coords[0] + $dx, next_coords[1] + $dy];
            value = get_neighbours(next_coords, &spiral);
            if value > limit {
                return value;
            }
            spiral.insert(next_coords, value);
        }
    }

    loop {
        next_check_limit!(1, 0);

        for _ in 0..q - 2 {
            next_check_limit!(0, -1);
        }

        for _ in 0..q - 1 {
            next_check_limit!(-1, 0);
        }

        for _ in 0..q - 1 {
            next_check_limit!(0, 1);
        }

        for _ in 0..q - 1 {
            next_check_limit!(1, 0);
        }
        // if q == 5 {
        //     println!("{:?}", spiral);
        //     return 0;
        // }

        q += 2;
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 3: Spiral Memory ---");

    let input_data = 325489;

    println!("{}", part1(input_data));
    println!("{}", part2(input_data));

    Ok(())
}
