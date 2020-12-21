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

    // Return neighbors in clockwise order
    fn neighbors(&self, i: i64) -> [char; 8] {
        let sget = |row, col| {
            if row < 0 || col < 0 || col >= self.width {
                '.'
            } else {
                self.seats
                    .get((row * self.width + col) as usize)
                    .copied()
                    .unwrap_or('.')
            }
        };
        let row = i / self.width;
        let col = i - row * self.width;
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
    }

    fn total_occupied(&self) -> usize {
        self.seats.iter().filter(|&&c| c == '#').count()
    }

    fn adj_occupied(&self, i: i64) -> usize {
        self.neighbors(i).iter().filter(|&&c| c == '#').count()
    }

    fn next_state(&self) -> Seats {
        Seats {
            width: self.width,
            seats: self
                .seats
                .iter()
                .enumerate()
                .map(|(i, &c)| match c {
                    'L' if self.adj_occupied(i as i64) == 0 => '#',
                    '#' if self.adj_occupied(i as i64) > 3 => 'L',
                    s => s,
                })
                .collect(),
        }
    }
}

pub fn part_a(data: String) -> Result<usize, Error> {
    let seats = Seats::new(&data);
    let final_state = std::iter::successors(Some(seats), |s| Some(s.next_state()))
        .map(std::rc::Rc::new)
        .tuple_windows()
        .find(|(a, b)| *a == *b)
        .unwrap()
        .0;
    Ok(final_state.total_occupied())
}

pub fn part_b(data: String) -> Result<i64, Error> {
    Ok(0)
}
