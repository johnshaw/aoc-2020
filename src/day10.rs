use failure::{bail, Error};
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_a(data: String) -> Result<i64, Error> {
    let mut adapters: Vec<i64> = data.lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    let diffs =
        adapters
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .fold(HashMap::new(), |mut m, d| {
                *m.entry(d).or_insert(0i64) += 1;
                m
            });
    Ok(diffs.get(&1).unwrap() * diffs.get(&3).unwrap())
}

pub fn part_b(data: String) -> Result<i64, Error> {
    let mut adapters: Vec<i64> = data.lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    let mut diffs: Vec<_> = adapters
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect();

    diffs.insert(0, 0);
    diffs.insert(0, 0);
    Ok(diffs
        .iter()
        .tuple_windows()
        .fold([1i64, 1, 1], |p, prev| match prev {
            (1, 1, 1) => [p[1], p[2], p[0] + p[1] + p[2]],
            (_, 1, 2) | (_, 2, 1) | (_, 1, 1) => [p[1], p[2], p[1] + p[2]],
            _ => [p[1], p[2], p[2]],
        })[2])
}
