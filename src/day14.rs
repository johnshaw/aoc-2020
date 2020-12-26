use failure::Error;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    // mask, set
    Mask(u64, u64),
    Store(u64, u64),
}

fn parse(code: &str) -> Vec<Op> {
    let re = Regex::new(r"(?:mask = (\w+))|(?:mem\[(\d+)\] = (\d+))").unwrap();
    code.lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            match caps.get(1) {
                Some(mask) => {
                    let (a, b) =
                        mask.as_str()
                            .chars()
                            //.enumerate()
                            .fold((0u64, 0u64), |(a, b), c| match c {
                                'X' => ((a << 1) | 1, b << 1),
                                '0' => (a << 1, b << 1),
                                '1' => (a << 1, (b << 1) | 1),
                                _ => panic!("invalid char"),
                            });
                    Op::Mask(a, b)
                }
                None => Op::Store(caps[2].parse().unwrap(), caps[3].parse().unwrap()),
            }
        })
        .collect()
}

pub fn part_a(data: String) -> Result<u64, Error> {
    let ops = parse(&data);
    let mut mem = HashMap::new();
    let mut mask = (0, 0);

    for op in ops {
        match op {
            Op::Mask(m, s) => {
                mask = (m, s);
            }
            Op::Store(addr, value) => {
                mem.insert(addr, mask.1 | (value & mask.0));
            }
        }
    }

    Ok(mem.values().sum())
}

fn bit_combs(n: u64) -> Vec<u64> {
    if n == 0 {
        vec![0]
    } else {
        let pos = 64 - n.leading_zeros() - 1;
        let mut v = bit_combs(n & !(1 << pos));
        for i in 0..v.len() {
            v.push(v[i] | (1 << pos));
        }
        v
    }
}

pub fn part_b(data: String) -> Result<u64, Error> {
    let ops = parse(&data);
    let mut mem = HashMap::new();
    let mut mask = (0, 0);

    for op in ops {
        match op {
            Op::Mask(m, s) => {
                mask = (m, s);
            }
            Op::Store(addr, value) => {
                let masks = bit_combs(mask.0);
                for m in masks {
                    let addr = (!mask.0 & addr) | m | mask.1;
                    mem.insert(addr, value);
                }
            }
        }
    }

    Ok(mem.values().sum())
}
