use failure::{bail, Error};
use itertools::Itertools;
use std::collections::HashSet;

pub fn part_a(data: String) -> Result<i64, Error> {
    let nums: HashSet<i64> = data
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    for n in &nums {
        let rem = 2020 - n;
        if nums.contains(&rem) {
            return Ok(n * rem);
        }
    }
    bail!("No answer");
}

pub fn part_b(data: String) -> Result<i64, Error> {
    let n = data
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == 2020)
        .map(|(a, b, c)| a * b * c)
        .unwrap();
    Ok(n)
}