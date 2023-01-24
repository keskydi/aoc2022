use std::{cmp, collections::BTreeSet, fmt, fs, str::FromStr};

type R = usize;

const SOURCE: Point = Point { x: 500, y: 0 };

fn level_1(input: &str) -> R {
    let mut i = parse_input(&input);
    let bottom = i.height.1;
    i.position(|sand| sand.y >= bottom).unwrap()
}

fn level_2(input: &str) -> R {
    let mut i = parse_input(&input);
    i.position(|sand| sand == SOURCE).unwrap() + 1
}

struct Grid {
    // Memory of trajectory
    structure: BTreeSet<Point>,
    mem: Vec<Point>,
    width: (u32, u32),
    height: (u32, u32),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();

        Ok(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

impl Iterator for Grid {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sand = self.mem.pop()?;

        loop {
            if sand.y > self.height.1 {
                self.structure.insert(sand);
                return Some(sand);
            }
            let options = [sand.x, sand.x - 1, sand.x + 1];

            let next_x = options
                .iter()
                .find(|&&x| !self.structure.contains(&Point { x, y: sand.y + 1 }));

            if let Some(&x) = next_x {
                self.mem.push(sand);
                sand = Point { x, y: sand.y + 1 }
            } else {
                self.structure.insert(sand);
                return Some(sand);
            }
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in 0..=self.height.1 + 5 {
            for x in self.width.0..=self.width.1 {
                if self.structure.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Grid {
    let mut walls = BTreeSet::new();

    for line in input.lines() {
        let mut coordinates = line.split(" -> ");

        let mut a: Point = coordinates.next().unwrap().parse().unwrap();

        while let Some(nxt) = coordinates.next() {
            let b: Point = nxt.parse().unwrap();
            for x in cmp::min(a.x, b.x)..=cmp::max(a.x, b.x) {
                for y in cmp::min(a.y, b.y)..=cmp::max(a.y, b.y) {
                    walls.insert(Point { x, y });
                }
            }

            a = b;
        }
    }
    walls.insert(SOURCE);

    let mut min_width = u32::MAX;
    let mut max_width = u32::MIN;
    let mut min_height = u32::MAX;
    let mut max_height = u32::MIN;
    for p in &walls {
        min_width = cmp::min(min_width, p.x);
        max_width = cmp::max(max_width, p.x);
        min_height = cmp::min(min_height, p.y);
        max_height = cmp::max(max_height, p.y);
    }

    Grid {
        mem: vec![SOURCE],
        structure: walls,
        width: (min_width - 15, max_width + 15),
        height: (min_height, max_height),
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day14/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}
