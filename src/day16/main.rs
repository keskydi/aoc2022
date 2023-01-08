// #![allow(warnings)]
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    error::Error,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

type R = String;

fn level_1(input: &str) -> R {
    unimplemented!()
}

fn level_2(input: &str) -> R {
    unimplemented!()
}

struct Graph<'a> {
    points: HashMap<&'a str, Node<'a>>,
    // points: HashMap<&'a str, (u32, Vec<&'a str>)>,
}

#[derive(Debug)]
struct Node<'a> {
    flow_rate: u32,
    edges: Vec<&'a str>,
    dist_edges: RefCell<Vec<(&'a str, u32)>>,
}

impl<'a> Node<'a> {
    fn new(flow_rate: u32, edges: Vec<&'a str>) -> Self {
        Node {
            flow_rate,
            edges,
            dist_edges: RefCell::new(Vec::new()),
        }
    }
}

impl<'a> Graph<'a> {
    // return sum of pressure release
    fn minimax(&self) -> u32 {
        // self.traverse_alone("AA", vec![], 31)
        self.traverse_with_elephant("AA", "AA", vec![], 27, 27)
    }

    fn resolve_edges_distances(&mut self) {
        // let keys = self
        //     .points
        //     .iter()
        //     .filter_map(|(&k, v)| if v.flow_rate > 0 { Some(k) } else { None })
        //     .collect::<Vec<&str>>();

        for (key, node) in self.points.iter() {
            let mut q: VecDeque<(&str, u32)> = node.edges.iter().map(|&x| (x, 1)).collect();
            let mut vst: Vec<&str> = vec![key];

            while let Some((id, dist)) = q.pop_front() {
                vst.push(id);
                let current_node = self.points.get(id).unwrap();
                let mut q1: VecDeque<(&str, u32)> = current_node
                    .edges
                    .iter()
                    .filter_map(|&x| {
                        if !vst.contains(&x) {
                            Some((x, dist + 1))
                        } else {
                            None
                        }
                    })
                    .collect();
                q.append(&mut q1);

                if current_node.flow_rate > 0 {
                    node.dist_edges.borrow_mut().push((id, dist));
                }
            }
        }
    }

    fn traverse_alone(&self, node: &'a str, mut vst: Vec<&'a str>, days_left: u32) -> u32 {
        let current = self.points.get(node).unwrap();

        vst.push(node);

        let pressure_release = current.flow_rate * (days_left - 1);

        current
            .dist_edges
            .borrow()
            .iter()
            .map(|(edge, dist)| {
                if (dist + 1) < days_left && !vst.contains(edge) {
                    self.traverse_alone(edge, vst.clone(), days_left - dist - 1)
                } else {
                    0_u32
                }
            })
            .max()
            .unwrap()
            + pressure_release
    }

    fn traverse_with_elephant(
        &self,
        node1: &'a str,
        node2: &'a str,
        mut vst: Vec<&'a str>,
        days_left1: u32,
        days_left2: u32,
    ) -> u32 {
        let c1 = self.points.get(node1).unwrap();
        let c2 = self.points.get(node2).unwrap();

        vst.push(node1);
        vst.push(node2);

        let p1 = c1.flow_rate * (days_left1 - 1);
        let p2 = c2.flow_rate * (days_left2 - 1);

        c1.dist_edges
            .borrow()
            .iter()
            .map(|(e1, d1)| {
                if !vst.contains(e1){
                c2.dist_edges
                    .borrow()
                    .iter()
                    .map(|(e2, d2)| {
                        if e1 != e2  && !vst.contains(e2) {
                            match ((d1 + 1) < days_left1, (d2 + 1) < days_left2) {
                                (true, true) => self.traverse_with_elephant(
                                    e1,
                                    e2,
                                    vst.clone(),
                                    0.max(days_left1 - d1 - 1),
                                    0.max(days_left2 - d2 - 1),
                                ),
                                (true, false) => {
                                    self.traverse_alone(e1, vst.clone(), days_left1 - d1 - 1)
                                }
                                (false, true) => {
                                    self.traverse_alone(e2, vst.clone(), days_left2 - d2 - 1)
                                }
                                (false, false) => 0,
                            }
                        } else {
                            0_u32
                        }
                    })
                    .max()
                    .unwrap()}else{
                        0
                    }
            })
            .max()
            .unwrap()
            + p1
            + p2
    }
}

fn parse_input<'a>(s: &'a str) -> IResult<&'a str, Graph<'a>> {
    separated_list1(
        line_ending,
        separated_pair(
            separated_pair(
                preceded(tag::<_, _, Error<_>>("Valve "), alpha1),
                tag(" has flow rate="),
                complete::u32,
            ),
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list0(tag(", "), alpha1),
        )
        .map(|((ident, rate), edges)| (ident, Node::new(rate, edges))),
    )(s)
    .map(|(s, points)| {
        (
            s,
            Graph {
                points: points.into_iter().collect::<HashMap<&str, Node<'_>>>(),
            },
        )
    })
}
fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day16/input.in").expect("file not found");

    let (_, mut graph) = parse_input(&input).unwrap();

    graph.resolve_edges_distances();

    println!("level_1: {}", graph.minimax());
    // println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}
