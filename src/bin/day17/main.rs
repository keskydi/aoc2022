use std::{collections::HashMap, fmt, fs};

struct Point {
    x: usize,
    y: usize,
}

const WIDTH: usize = 7;

const SHAPES: [&[Point]; 5] = [
    &[
        Point { x: 0, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: 2 },
        Point { x: 0, y: 3 },
    ],
    &[
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 2, y: 1 },
        Point { x: 1, y: 2 },
    ],
    &[
        Point { x: 0, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 2, y: 2 },
    ],
    &[
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 3, y: 0 },
    ],
    &[
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
    ],
];

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::Right,
            '<' => Self::Left,
            _ => panic!("invalid input, {}", c),
        }
    }
}

#[derive(Default)]
struct State {
    map: Vec<[bool; WIDTH + 1]>,
    top: usize,
}

impl State {
    fn is_valid(&mut self, coord: &Point, shape: &[Point]) -> bool {
        shape.iter().all(|offset| {
            let x = offset.x + coord.x;
            let y = offset.y + coord.y;

            while self.map.len() <= x {
                self.map.push([false; WIDTH + 1]);
            }

            y < WIDTH && !self.map[x][y]
        })
    }

    fn signature(&self) -> u128 {
        let rows: Vec<u128> = self
            .map
            .iter()
            .rev()
            .take(16)
            .flatten()
            .map(|&x| x.into())
            .collect();

        let mut signature = 0;
        for (i, &x) in rows.iter().enumerate() {
            signature |= x << i;
        }
        signature
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;

        for i in self.map.iter().rev() {
            write!(f, "|")?;
            for &y in i {
                if y {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")?;
        Ok(())
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/bin/day17/input.in").expect("file not found");

    let jet_pattern: Vec<Jet> = input.trim().chars().map(Jet::from).collect();

    let jet_lenght = jet_pattern.len();

    let mut status = State::default();

    let mut current_shape = 0;
    let mut current_jet = 0;
    let mut delta = 0;

    let mut seen = HashMap::new();

    let lenght = 1_000_000_000_000;
    while current_shape < lenght {
        let mut current_coord = Point {
            x: status.top + 3,
            y: 2,
        };

        loop {
            let j = &jet_pattern[current_jet % jet_lenght];

            let next_coord = Point {
                x: current_coord.x,
                y: {
                    match j {
                        Jet::Left => current_coord.y.saturating_sub(1),
                        Jet::Right => current_coord.y + 1,
                    }
                },
            };

            if status.is_valid(&next_coord, SHAPES[current_shape % 5]) {
                current_coord = next_coord;
            }
            current_jet += 1;

            let next_coord = Point {
                x: current_coord.x.saturating_sub(1),
                y: current_coord.y,
            };

            if current_coord.x == 0 || !status.is_valid(&next_coord, SHAPES[current_shape % 5]) {
                break;
            }

            current_coord = next_coord;
        }

        SHAPES[current_shape % 5].iter().for_each(|offset| {
            let x = offset.x + current_coord.x;
            let y = offset.y + current_coord.y;

            while status.map.len() <= x {
                status.map.push([false; WIDTH + 1]);
            }

            status.top = status.top.max(x + 1);
            status.map[x][y] = true;
        });

        if let Some(value) = seen.insert(
            (
                current_jet % jet_lenght,
                current_shape % 5,
                status.signature(),
            ),
            (current_shape, status.top),
        ) {
            if current_shape > 2022 {
                let (oldt, oldy) = value;
                let dy = current_shape - oldt;
                let dt = status.top - oldy;
                let amt = (lenght - current_shape).div_euclid(dy);

                delta += amt * dt;
                current_shape += amt * dy;
            }
        }

        current_shape += 1;
        if current_shape == 2022 {
            println!("part_1: {:?}", status.top);
        }
    }

    println!("part_2: {:?}", status.top + delta);
    println!("time: {:?}", now.elapsed());
}
