use failure::Error;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Seats {
    width: i64,
    // row-major order
    seats: Vec<char>,
}

impl Seats {
    fn new(data: &str) -> Seats {
        Seats {
            width: data.lines().next().unwrap().len() as i64,
            seats: data.lines().map(|l| l.chars()).flatten().collect(),
        }
    }

    fn total_occupied(&self) -> usize {
        self.seats.iter().filter(|&&c| c == '#').count()
    }

    fn next_state(&self, rule: fn(&Seats, usize) -> char) -> Seats {
        Seats {
            width: self.width,
            seats: (0..self.seats.len()).map(|i| rule(&self, i)).collect(),
        }
    }
}

fn solve(seats: Seats, rule: fn(&Seats, usize) -> char) -> Seats {
    std::iter::successors(Some(seats), |s| Some(s.next_state(rule)))
        .tuple_windows()
        .find(|(a, b)| *a == *b)
        .unwrap()
        .0
}

fn rule_parta(seats: &Seats, pos: usize) -> char {
    // Return neighbors in clockwise order
    let neighbors = |i: i64| {
        let sget = |row, col| {
            if row < 0 || col < 0 || col >= seats.width {
                '.'
            } else {
                seats
                    .seats
                    .get((row * seats.width + col) as usize)
                    .copied()
                    .unwrap_or('.')
            }
        };
        let row = i / seats.width;
        let col = i - row * seats.width;
        [
            sget(row - 1, col - 1),
            sget(row - 1, col),
            sget(row - 1, col + 1),
            sget(row, col + 1),
            sget(row + 1, col + 1),
            sget(row + 1, col),
            sget(row + 1, col - 1),
            sget(row, col - 1),
        ]
    };

    let adj_occupied = |i: i64| neighbors(i).iter().filter(|&&c| c == '#').count();

    match seats.seats[pos] {
        'L' if adj_occupied(pos as i64) == 0 => '#',
        '#' if adj_occupied(pos as i64) > 3 => 'L',
        s => s,
    }
}

pub fn part_a(data: String) -> Result<usize, Error> {
    let seats = Seats::new(&data);
    Ok(solve(seats, rule_parta).total_occupied())
}

fn rule_partb(seats: &Seats, pos: usize) -> char {
    // Return neighbors in clockwise order
    let neighbors = |i: i64| {
        let search = |ir, ic| {
            let mut row = i / seats.width;
            let mut col = i - row * seats.width;
            loop {
                row += ir;
                col += ic;
                if row < 0 || col < 0 || col >= seats.width {
                    break '.'
                } else {
                    match seats.seats.get((row * seats.width + col) as usize).copied() {
                        Some(c) if c != '.' => break c,
                        None => break '.',
                        _ => {},
                    }
                }
            }
        };
        [
            search(-1, -1),
            search(-1, 0),
            search(-1, 1),
            search(0, 1),
            search(1, 1),
            search(1, 0),
            search(1, -1),
            search(0, -1),
        ]
    };

    let adj_occupied = |i: i64| neighbors(i).iter().filter(|&&c| c == '#').count();

    match seats.seats[pos] {
        'L' if adj_occupied(pos as i64) == 0 => '#',
        '#' if adj_occupied(pos as i64) > 4 => 'L',
        s => s,
    }
}

pub fn part_b(data: String) -> Result<usize, Error> {
    let seats = Seats::new(&data);
    Ok(solve(seats, rule_partb).total_occupied())
}
