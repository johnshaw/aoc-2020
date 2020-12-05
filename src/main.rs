mod util;

mod day01;
mod day02;
mod day03;

use util::ProblemPart::*;
use util::*;

use failure::Error;

fn run(
    day: u32,
    part: ProblemPart,
    func: fn(String) -> Result<String, Error>,
) -> Result<(), Error> {
    let input = read_input(day)?;
    let result = func(input)?;
    println!("day {} part {}: {}", day, part.letter(), result);
    Ok(())
}

fn main() -> Result<(), Error> {
    run(1, First, day01::part_a)?;
    run(1, Second, day01::part_b)?;
    run(2, First, day02::part_a)?;
    run(2, Second, day02::part_b)?;
    run(3, First, day03::part_a)?;
    run(3, Second, day03::part_b)?;
    Ok(())
}
