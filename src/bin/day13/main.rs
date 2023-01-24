use nom::{
    branch::alt, bytes::complete::tag, character::complete, multi::separated_list0,
    sequence::delimited, IResult, Parser,
};
use std::{cmp::Ordering, fs};

type R = usize;

#[derive(Clone)]
enum Element {
    Number(u32),
    Array(Vec<Element>),
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0.cmp(r0),
            (Self::Array(l0), Self::Array(r0)) => l0.cmp(&r0),
            (Self::Array(l0), Self::Number(r0)) => l0.cmp(&vec![Self::Number(*r0)]),
            (Self::Number(l0), Self::Array(r0)) => vec![Self::Number(*l0)].cmp(r0),
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => l0 == r0,
            (Self::Array(l0), Self::Number(r0)) => l0 == &vec![Self::Number(*r0)],
            (Self::Number(l0), Self::Array(r0)) => &vec![Self::Number(*l0)] == r0,
        }
    }
}

impl Eq for Element {}

fn level_1(input: &str) -> R {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, c)| {
            let mut lines = c.lines();
            let (Some(a),Some(b)) =  ((lines.next()),lines.next()) else{
                panic!("error parsing")
            };
            let a = parse_input(a).unwrap().1;
            let b = parse_input(b).unwrap().1;

            if a.cmp(&b) == Ordering::Less {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}

fn level_2(input: &str) -> R {
    let mut list: Vec<Element> = input
        .split("\n\n")
        .map(|c| {
            let mut lines = c.lines();
            let (Some(a),Some(b)) =  ((lines.next()),lines.next()) else{
                panic!("error parsing")
            };
            let a = parse_input(a).unwrap().1;
            let b = parse_input(b).unwrap().1;

            vec![a, b]
        })
        .flatten()
        .collect();

    let b1 = Element::Array(vec![Element::Array(vec![Element::Number(2)])]);
    let b2 = Element::Array(vec![Element::Array(vec![Element::Number(6)])]);
    list.push(b1.clone());
    list.push(b2.clone());

    list.sort();

    (list.iter().position(|x| x == &b1).unwrap() + 1)
        * (list.iter().position(|x| x == &b2).unwrap() + 1)
}

fn parse_input(input: &str) -> IResult<&str, Element> {
    alt((
        delimited(tag("["), separated_list0(tag(","), parse_input), tag("]"))
            .map(|values| Element::Array(values)),
        complete::u32.map(|value| Element::Number(value)),
    ))(input)
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day13/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}
