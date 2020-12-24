use std::collections::HashSet;
use failure::Error;
use num::Integer;

pub fn part_a(data: String) -> Result<i64, Error> {
    let arrival: i64 = data.lines().nth(0).unwrap().parse().unwrap();
    let buses: HashSet<i64> = data
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .collect();
    let (id, wait) = buses
        .iter()
        .map(|b| (*b, *b - (arrival % *b)))
        .min_by_key(|(_, d)| *d)
        .unwrap();
    Ok(id * wait)
}

pub fn part_b(data: String) -> Result<i64, Error> {
    let buses: Vec<Option<i64>> = data
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| match s {
            "x" => None,
            n => Some(n.parse().unwrap()),
        })
        .collect();

    let N: i64 = buses.iter().filter_map(|x| *x).product();

    let res: i64 = buses
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.map(|n| (i as i64, n)))
        .map(|(a, n)| (n - a, n))
        .map(|(a, n)| {
            let Ni = N / n;
            let egcd = Ni.extended_gcd(&n);
            a*Ni*(if egcd.x < 0 { egcd.x + n } else {egcd.x })
        })
        .sum();

    //println!("{:?}", res % N);
    Ok(res % N)
}
