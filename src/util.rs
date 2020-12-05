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
