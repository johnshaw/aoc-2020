use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Bag<'a> {
    name: &'a str,
    allowed: Vec<(usize, &'a str)>,
}

fn parse(data: &str) -> Vec<Bag<'_>> {
    let bag_re = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    let mut bags = Vec::new();
    for line in data.lines() {
        let (outer, inner) = line.split(" bags contain ").collect_tuple().unwrap();
        if inner == "no other bags." {
            bags.push(Bag {
                name: outer,
                allowed: Vec::new(),
            });
        } else {
            bags.push(Bag {
                name: outer,
                allowed: inner
                    .split(", ")
                    .map(|b| bag_re.captures(b).unwrap())
                    .map(|c| {
                        (
                            c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            c.get(2).unwrap().as_str(),
                        )
                    })
                    .collect(),
            });
        }
    }
    bags
}

fn dfs<'a>(edges: &HashMap<&'a str, Vec<&'a str>>, item: &'a str) -> HashSet<&'a str> {
    let mut res = HashSet::new();
    res.insert(item);
    match edges.get(item) {
        Some(v) => {
            for i in v {
                res.extend(dfs(edges, i));
            }
        }
        None => {}
    }
    res
}

pub fn part_a(data: String) -> Result<usize, Error> {
    let bags = parse(&data);
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for bag in bags.iter() {
        for child in bag.allowed.iter() {
            edges.entry(child.1).or_default().push(bag.name);
        }
    }
    let reachable = dfs(&edges, "shiny gold");
    Ok(reachable.len() - 1)
}

pub fn part_b(data: String) -> Result<usize, Error> {
    let bags = parse(&data);

    let mut edges: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
    for bag in bags.into_iter() {
        if !bag.allowed.is_empty() {
            edges.insert(bag.name, bag.allowed);
        }
    }

    fn traverse<'a>(edges: &HashMap<&'a str, Vec<(usize, &'a str)>>, item: &'a str) -> usize {
        match edges.get(item) {
            Some(v) => v
                .iter()
                .map(|(count, child)| count * traverse(edges, child))
                .sum::<usize>() + 1,
            None => 1,
        }
    }

    Ok(traverse(&edges, "shiny gold") - 1)
}
