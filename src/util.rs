use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, Debug)]
pub enum ProblemPart {
    First,
    Second,
}

impl ProblemPart {
    pub fn letter(&self) -> char {
        match self {
            Self::First => 'a',
            Self::Second => 'b',
        }
    }
}

// Read input file into a string
pub fn read_input(day: u32) -> std::io::Result<String> {
    assert!(day <= 25);
    let path = format!("data/day{:0>2}.txt", day);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

trait StrStuff {
    fn split_one<'a>(&'a self, pat: &str) -> Option<(&'a str, &'a str)>;
    fn split_whitespace_one<'a>(&'a self) -> Option<(&'a str, &'a str)>;
}

impl StrStuff for str {
    fn split_one<'a>(&'a self, pat: &str) -> Option<(&'a str, &'a str)> {
        let mut parts = self.split(pat);
        let p1 = parts.next();
        let p2 = parts.next();
        p1.and_then(|a| p2.map(|b| (a, b)))
    }

    fn split_whitespace_one<'a>(&'a self) -> Option<(&'a str, &'a str)> {
        let mut parts = self.split_whitespace();
        let p1 = parts.next();
        let p2 = parts.next();
        p1.and_then(|a| p2.map(|b| (a, b)))
    }
}
