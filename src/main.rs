mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use util::ProblemPart::*;
use util::*;

use failure::Error;

fn run<T: std::fmt::Display>(
    day: u32,
    part: ProblemPart,
    func: fn(String) -> Result<T, Error>,
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
    run(4, First, day04::part_a)?;
    run(4, Second, day04::part_b)?;
    run(5, First, day05::part_a)?;
    run(5, Second, day05::part_b)?;
    run(6, First, day06::part_a)?;
    run(6, Second, day06::part_b)?;
    run(7, First, day07::part_a)?;
    run(7, Second, day07::part_b)?;
    run(8, First, day08::part_a)?;
    run(8, Second, day08::part_b)?;
    run(9, First, day09::part_a)?;
    run(9, Second, day09::part_b)?;
    Ok(())
}
