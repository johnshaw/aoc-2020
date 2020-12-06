use failure::Error;
use itertools::Itertools;
use std::collections::HashSet;

// Collect each group into sets of answered questions, one set for each person in the group
fn parse_input(data: &str) -> impl Iterator<Item = Vec<HashSet<char>>> + '_ {
    data.split_terminator("\n\n")
        .map(|group| group.lines().map(|l| l.chars().collect()).collect())
}

fn set_fold<T, U>(sets: T, op: fn(&HashSet<U>, &HashSet<U>) -> HashSet<U>) -> HashSet<U>
where
    T: IntoIterator<Item = HashSet<U>>,
    U: Eq + std::hash::Hash + Clone,
{
    sets.into_iter().fold1(|a, b| op(&a, &b)).unwrap()
}

pub fn part_a(data: String) -> Result<usize, Error> {
    Ok(parse_input(&data)
        .map(|g| set_fold(g, |a, b| a | b))
        .map(|s| s.len())
        .sum())
}

pub fn part_b(data: String) -> Result<usize, Error> {
    Ok(parse_input(&data)
        .map(|g| set_fold(g, |a, b| a & b))
        .map(|s| s.len())
        .sum())
}
