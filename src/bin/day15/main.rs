use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use std::{collections::HashSet, fs};

type R = usize;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    fn intersect(&self, other: &Segment) -> Option<Point> {
        let (a, b) = self.function();
        let (c, d) = other.function();
        if (b + d) % 2 == 1 || a * c != -1 {
            None
        } else {
            let x = (d - b) / (a - c);
            if self.a.x < x && x < self.b.x {
                Some(Point { x, y: a * x + b })
            } else {
                None
            }
        }
    }

    fn function(&self) -> (i32, i32) {
        let a = (self.a.y < self.b.y) as i32 * 2 - 1;
        (a, self.a.y - a * self.a.x)
    }
}

impl From<((Point, i32), (Point, i32))> for Segment {
    fn from(points: ((Point, i32), (Point, i32))) -> Self {
        let (a, b) = points;

        match (a.0.x > b.0.x, a.0.y > b.0.y) {
            // b is SW
            (true, true) => Segment {
                a: Point {
                    x: a.0.x - a.1,
                    y: a.0.y,
                },
                b: Point {
                    x: a.0.x,
                    y: a.0.y - a.1,
                },
            },
            // b is SE
            (true, false) => Segment {
                a: Point {
                    x: a.0.x - a.1,
                    y: a.0.y,
                },
                b: Point {
                    x: a.0.x,
                    y: a.0.y + a.1,
                },
            },
            // b is NW
            (false, true) => Segment {
                a: Point {
                    x: a.0.x,
                    y: a.0.y - a.1,
                },
                b: Point {
                    x: a.0.x + a.1,
                    y: a.0.y,
                },
            },
            // b is NE
            (false, false) => Segment {
                a: Point {
                    x: a.0.x,
                    y: a.0.y + a.1,
                },
                b: Point {
                    x: a.0.x + a.1,
                    y: a.0.y,
                },
            },
        }
    }
}

#[test]
fn test_segment() {
    let a = Point { x: 2, y: 4 };
    let b = Point { x: 6, y: 2 };

    let segment: Segment = ((a, 3), (b, 3)).into();

    assert_eq!(
        segment,
        Segment {
            a: Point { x: 2, y: 1 },
            b: Point { x: 5, y: 4 }
        }
    )
}

#[test]
fn test_manhattan() {
    let a = Point { x: 12, y: 14 };
    let b = Point { x: 10, y: 16 };
    assert_eq!(4, a.manhattan(&b));

    let c = Point { x: 16, y: 7 };
    let d = Point { x: 15, y: 3 };
    assert_eq!(5, c.manhattan(&d));
    assert_eq!(a.manhattan(&b) + c.manhattan(&d) + 2, a.manhattan(&c));
}

impl Point {
    fn manhattan(&self, o: &Point) -> i32 {
        (self.x - o.x).abs() + (self.y - o.y).abs()
    }
}

fn level_1(input: &str, y: u32) -> R {
    let list = parse_input(input).unwrap().1;
    let mut points = HashSet::new();

    for (sensor, beacon) in list {
        let radius = sensor.manhattan(&beacon);

        for x in sensor.x - radius..=sensor.x + radius {
            let point = Point { x, y: y as i32 };
            if sensor.manhattan(&point) <= radius && point != beacon {
                points.insert(point);
            }
        }
    }
    points.len()
}

fn level_2(input: &str) -> R {
    let list = parse_input(input).unwrap().1;

    let mut segments: Vec<Segment> = Vec::new();

    (1..list.len()).for_each(|i| {
        let current = &list[i - 1];
        let r = current.0.manhattan(&current.1);

        list[i..].iter().for_each(|(s, b)| {
            let r1 = s.manhattan(b);
            let d = current.0.manhattan(s);
            if d == r1 + r + 2 {
                segments.push(((current.0, r + 1), (*s, r1 + 1)).into())
            }
        })
    });

    let mut points = HashSet::new();
    (1..segments.len()).for_each(|i| {
        let c = &segments[i - 1];

        segments[i..].iter().for_each(|s| {
            if let Some(p) = c.intersect(s) {
                points.insert(p);
            }
        })
    });

    for p in points {
        if list.iter().all(|(s, b)| {
            let r = s.manhattan(b);
            let d = p.manhattan(s);
            r < d
        }) {
            return (p.x as usize * 4000000) + p.y as usize;
        }
    }

    0
}

fn parse_position(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Point, Point)>> {
    separated_list1(
        line_ending,
        preceded(
            tag("Sensor at "),
            separated_pair(
                parse_position.map(|(x, y)| Point { x, y }),
                tag(": closest beacon is at "),
                parse_position.map(|(x, y)| Point { x, y }),
            ),
        ),
    )(input)
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/bin/day15/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input,2000000));
    println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}
