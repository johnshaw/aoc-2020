use failure::{bail, Error};
use lazy_static::lazy_static;
use regex::Regex;

struct Map {
    width: usize,
    height: usize,
    // Stored in row major order. True is tree
    trees: Vec<bool>,
}

impl Map {
    fn parse(data: &str) -> Map {
        let mut height = 0usize;
        let mut trees = Vec::new();

        for row in data.lines().filter(|s| !s.is_empty()) {
            trees.extend(row.chars().map(|c| c == '#'));
            height += 1;
        }

        Map {
            width: trees.len() / height,
            height,
            trees,
        }
    }

    // [0,0] is top left corner. Map has infinite width. Returns None if y is out of range.
    fn has_tree(&self, x: usize, y: usize) -> Option<bool> {
        if y >= self.height {
            None
        } else {
            Some(self.trees[y * self.width + x % self.width])
        }
    }
}

fn count_trees(map: &Map, step_x: usize, step_y: usize) -> usize {
    (0..)
        .step_by(step_x)
        .zip((0..).step_by(step_y))
        .map(|(x, y)| map.has_tree(x, y))
        .take_while(|t| t.is_some())
        .flatten()
        .filter(|b| *b)
        .count()
}

pub fn part_a(data: String) -> Result<String, Error> {
    let map = Map::parse(&data);
    Ok(format!("{}", count_trees(&map, 3, 1)))
}

pub fn part_b(data: String) -> Result<String, Error> {
    let map = Map::parse(&data);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    Ok(format!(
        "{}",
        slopes
            .iter()
            .map(|(x, y)| count_trees(&map, *x, *y))
            .fold(1, |a, c| a * c)
    ))
}
