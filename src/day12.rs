use failure::Error;

pub fn part_a(data: String) -> Result<i64, Error> {
    // state is an angle and pos (angle, x, y)
    let (_, x, y) = data.lines().fold((90i64, 0i64, 0i64), |(a, x, y), l| {
        let v: i64 = l[1..].parse().unwrap();

        let mov = |d, v| match d {
            "N" => (a, x, y + v),
            "S" => (a, x, y - v),
            "E" => (a, x + v, y),
            "W" => (a, x - v, y),
            _ => panic!("bad direction"),
        };

        match &l[..1] {
            "L" => ((a - v) % 360, x, y),
            "R" => ((a + v) % 360, x, y),
            "F" => mov(
                match a {
                    0 => "N",
                    90 | -270 => "E",
                    180 | -180 => "S",
                    270 | -90 => "W",
                    _ => panic!(format!("bad angle: {}", a)),
                },
                v,
            ),
            d => mov(d, v),
        }
    });
    Ok(x.abs() + y.abs())
}

pub fn part_b(data: String) -> Result<i64, Error> {
    // state is an angle and pos (angle, x, y)
    let init = (
        // Ship location
        (0i64, 0i64),
        // Waypoint location (relative to ship)
        (10i64, 1i64),
    );

    let ((x, y), _) = data.lines().fold(init, |state, l| {
        let v: i64 = l[1..].parse().unwrap();

        let rot = |a: i64, x: i64, y: i64| match a {
            0 => (x, y),
            90 | -270 => (y, -x),
            180 | -180 => (-x, -y),
            270 | -90 => (-y, x),
            _ => panic!("bad angle"),
        };

        let ((sx, sy), (wx, wy)) = state;

        match &l[..1] {
            "N" => (state.0, (wx, wy + v)),
            "S" => (state.0, (wx, wy - v)),
            "E" => (state.0, (wx + v, wy)),
            "W" => (state.0, (wx - v, wy)),
            "L" => (state.0, rot(-v, wx, wy)),
            "R" => (state.0, rot(v, wx, wy)),
            "F" => ((sx + wx * v, sy + wy * v), state.1),
            _ => panic!("bad instruction"),
        }
    });
    Ok(x.abs() + y.abs())
}
