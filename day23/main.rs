use std::env;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day23: Coprocessor Conflagration ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    println!("{}", input_file);

    Ok(())
}
