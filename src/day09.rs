use failure::{bail, Error};
use itertools::Itertools;

pub fn part_a(data: String) -> Result<i64, Error> {
    let nums: Vec<i64> = data.lines().map(|n| n.parse().unwrap()).collect();
    for win in nums.windows(26) {
        let n = win.last().unwrap();
        let prev = &win[..25];
        if prev
            .iter()
            .combinations(2)
            .filter(|x| x.iter().copied().sum::<i64>() == *n)
            .count()
            == 0
        {
            return Ok(*n);
        }
    }
    bail!("No answer");
}

pub fn part_b(data: String) -> Result<i64, Error> {
    let target = part_a(data.clone()).unwrap();
    let nums: Vec<i64> = data.lines().map(|n| n.parse().unwrap()).collect();

    for x in 0..nums.len() {
        let mut s = 0i64;
        for y in x..nums.len() {
            s += nums[y];
            if s > target {
                break;
            } else if s == target {
                return Ok(nums[x..=y].iter().min().unwrap() + nums[x..=y].iter().max().unwrap());
            }
        }
    }

    bail!("No answer");
}
