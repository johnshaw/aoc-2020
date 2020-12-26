use failure::Error;
use std::collections::{HashMap, VecDeque};

fn parse(data: &str) -> Vec<i64> {
    data.split(',').map(|n| n.parse().unwrap()).collect()
}

fn play(mut nums: Vec<i64>, end: usize) -> i64 {
    let mut positions: HashMap<i64, VecDeque<usize>> = nums
        .iter()
        .enumerate()
        .map(|(i, &n)| {
            let mut v = VecDeque::new();
            v.push_front(i);
            (n, v)
        })
        .collect();
    let mut idx = nums.len();
    loop {
        let n = match positions.get(nums.last().unwrap()) {
            None => 0,
            Some(v) => {
                if v.len() > 1 {
                    (v[0] - v[1]) as i64
                } else {
                    0
                }
            }
        };
        positions.entry(n).or_default().push_front(idx);
        positions.entry(n).or_default().truncate(2);
        nums.push(n);
        idx += 1;

        if idx == end {
            break;
        }
    }
    *nums.last().unwrap()
}

pub fn part_a(data: String) -> Result<i64, Error> {
    Ok(play(parse(&data), 2020))
}

pub fn part_b(data: String) -> Result<i64, Error> {
    Ok(play(parse(&data), 30000000))
}
