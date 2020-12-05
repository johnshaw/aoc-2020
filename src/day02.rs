use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;

struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

impl Policy {
    fn is_valid_1(&self, pass: &str) -> bool {
        let count = pass.chars().filter(|c| *c == self.letter).count();
        count >= self.min && count <= self.max
    }

    fn is_valid_2(&self, pass: &str) -> bool {
        let a = pass.chars().nth(self.min - 1) == Some(self.letter);
        let b = pass.chars().nth(self.max - 1) == Some(self.letter);
        (a || b) && !(a && b)
    }
}

struct Entry<'a> {
    policy: Policy,
    passwd: &'a str,
}

lazy_static! {
    static ref ENTRY_REGEX: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

fn parse_input(data: &str) -> Result<impl Iterator<Item = Entry>, Error> {
    Ok(data
        .lines()
        .filter_map(|line| ENTRY_REGEX.captures(line))
        .map(|cap| Entry {
            policy: Policy {
                min: cap.get(1).unwrap().as_str().parse().unwrap(),
                max: cap.get(2).unwrap().as_str().parse().unwrap(),
                letter: cap.get(3).unwrap().as_str().chars().next().unwrap(),
            },
            passwd: cap.get(4).unwrap().as_str(),
        }))
}

pub fn part_a(data: String) -> Result<String, Error> {
    Ok(format!(
        "{}",
        parse_input(&data)?
            .filter(|e| e.policy.is_valid_1(e.passwd))
            .count()
    ))
}

pub fn part_b(data: String) -> Result<String, Error> {
    Ok(format!(
        "{}",
        parse_input(&data)?
            .filter(|e| e.policy.is_valid_2(e.passwd))
            .count()
    ))
}
