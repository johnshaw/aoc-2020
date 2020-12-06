use failure::Error;
use itertools::Itertools;

enum Dir {
    Left,
    Right,
}

fn bsp_eval<D>(mut begin: usize, mut end: usize, dirs: D) -> (usize, usize)
where
    D: IntoIterator<Item = Dir>,
{
    for d in dirs {
        let half_len = (end - begin) / 2;
        match d {
            Dir::Left => end -= half_len,
            Dir::Right => begin += half_len,
        }
    }
    (begin, end)
}

fn code_to_dir(c: char) -> Dir {
    match c {
        'F' | 'L' => Dir::Left,
        'B' | 'R' => Dir::Right,
        _ => panic!("Unexpected code character"),
    }
}

// Convert seat FBLR seat code to integer id
fn compute_seat_id(code: &str) -> usize {
    let (row_code, col_code) = code.split_at(7);
    let (rb, _) = bsp_eval(0, 128, row_code.chars().map(code_to_dir));
    let (cb, _) = bsp_eval(0, 8, col_code.chars().map(code_to_dir));
    rb * 8 + cb
}

pub fn part_a(data: String) -> Result<usize, Error> {
    Ok(data.split_whitespace().map(compute_seat_id).max().unwrap())
}

pub fn part_b(data: String) -> Result<usize, Error> {
    let mut ids: Vec<_> = data.split_whitespace().map(compute_seat_id).collect();
    ids.sort();

    let (prev, _) = ids
        .into_iter()
        .tuple_windows()
        .find(|(a, b)| *a + 2 == *b)
        .unwrap();
    Ok(prev + 1)
}
