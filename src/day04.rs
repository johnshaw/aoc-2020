use failure::Error;
use std::collections::HashMap;

// Parse passport fields into a hash map
fn parse_passport(data: &str) -> HashMap<&str, &str> {
    data.split_whitespace()
        .map(|e| e.find(':').map(|p| e.split_at(p)).unwrap())
        // Need to remove the leading ':' from value after the split. (split_once exists on nightly)
        .map(|(a, b)| (a, &b[1..]))
        .collect()
}

fn parse_input(data: &str) -> impl Iterator<Item = HashMap<&str, &str>> {
    data.split_terminator("\n\n").map(parse_passport)
}

fn has_required_fields(passport: &HashMap<&str, &str>) -> bool {
    let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_fields.iter().all(|f| passport.contains_key(f))
}

pub fn part_a(data: String) -> Result<usize, Error> {
    Ok(parse_input(&data).filter(has_required_fields).count())
}

fn is_field_valid(k: &str, v: &str) -> bool {
    fn validate_year(year: &str, first: u32, last: u32) -> bool {
        if year.len() == 4 {
            if let Ok(n) = year.parse::<u32>() {
                return n >= first && n <= last;
            }
        }
        false
    }

    match k {
        "byr" => validate_year(v, 1920, 2002),
        "iyr" => validate_year(v, 2010, 2020),
        "eyr" => validate_year(v, 2020, 2030),
        "hgt" => {
            if let Some(Ok(n)) = v.strip_suffix("in").map(str::parse::<u32>) {
                n >= 59 && n <= 76
            } else if let Some(Ok(n)) = v.strip_suffix("cm").map(str::parse::<u32>) {
                n >= 150 && n <= 193
            } else {
                false
            }
        }
        "hcl" => {
            v.len() == 7
                && v.chars().nth(0).unwrap() == '#'
                && v.chars().skip(1).all(|c| {
                    c.is_ascii_digit() || (c.is_ascii_hexdigit() && c.is_ascii_lowercase())
                })
        }
        "ecl" => match v {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => unreachable!(),
    }
}

fn is_valid(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(k, v)| is_field_valid(k, v))
}

pub fn part_b(data: String) -> Result<usize, Error> {
    Ok(parse_input(&data)
        .filter(has_required_fields)
        .filter(is_valid)
        .count())
}
