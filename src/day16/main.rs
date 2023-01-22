use aoc2022::Priority;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    error::Error,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs,
};
use hashbrown::{HashMap, HashSet};

fn parse_input(s: &str) -> IResult<&str, Vec<(&str, u16, Vec<&str>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            separated_pair(
                preceded(tag::<_, _, Error<_>>("Valve "), alpha1),
                tag(" has flow rate="),
                complete::u16,
            ),
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list0(tag(", "), alpha1),
        )
        .map(|((ident, rate), edges)| (ident, rate, edges)),
    )(s)
}

fn floyd_warshall(graph: &[(&str, u16, Vec<&str>)]) -> Vec<Vec<u16>> {
    let map = graph
        .iter()
        .enumerate()
        .map(|(i, &(id, _, _))| (id, i))
        .collect::<HashMap<&str, _>>();

    let n = graph.len();

    let mut dist = vec![vec![u16::MAX; n]; n];

    for (i, (_, _, edges)) in graph.iter().enumerate() {
        for &node in edges {
            let y = map[node];
            dist[i][y] = 1;
        }
        dist[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let (result, overflow) = dist[i][k].overflowing_add(dist[k][j]);
                if !overflow && dist[i][j] > result {
                    dist[i][j] = result;
                }
            }
        }
    }

    dist
}
fn main() {
    let now = std::time::Instant::now();
    let data = fs::read_to_string("src/day16/input.in").expect("file not found");

    // let (_, mut graph) = parse_input(&input).unwrap();
    let (_, rows) = parse_input(&data).unwrap();

    let dist = floyd_warshall(&rows);

    let useful_rows = rows
        .iter()
        .enumerate()
        .filter_map(|(i, &(id, flow, _))| (id == "AA" || flow > 0).then(|| i))
        .collect::<Vec<usize>>();

    let flow_rates: Vec<_> = useful_rows.iter().map(|&i| rows[i].1).collect();

    let filtered_dist: Vec<Vec<u16>> = useful_rows
        .iter()
        .map(|&x| useful_rows.iter().map(|&y| dist[x][y] + 1).collect())
        .collect();

    let starting_node = useful_rows.iter().position(|&i| rows[i].0 == "AA").unwrap();

    let best_valves: Vec<Vec<_>> = (0..=30)
        .map(|t| {
            flow_rates
                .iter()
                .enumerate()
                .filter_map(|(i, &flow)| {
                    let min_dist = *filtered_dist[i].iter().filter(|&&v| v > 0).min().unwrap();

                    (t > min_dist).then(|| (i, min_dist, flow))
                })
                .sorted_by_key(|&(_, min_dist, flow)| Reverse(flow * (t - min_dist)))
                .collect()
        })
        .collect();

    println!(
        "Part 1: {:?}",
        max_pressure_release(
            starting_node,
            [30, 0],
            &filtered_dist,
            &flow_rates,
            &best_valves
        )
    );
    println!(
        "Part 2: {:?}",
        max_pressure_release(
            starting_node,
            [26, 26],
            &filtered_dist,
            &flow_rates,
            &best_valves
        )
    );

    println!("Time: {:?}", now.elapsed());
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    visited: u16,
    time_left: [u16; 2],
    position: [usize; 2],
    presure_released: u16,
}

impl State {
    fn calc_upper(&self, best_valves: &[Vec<(usize, u16, u16)>]) -> u16 {
        let [mut t1, mut t2] = self.time_left;
        let mut visited = self.visited;
        let mut bound = self.presure_released;

        'next_valve: loop {
            for (i, min_dist, flow) in &best_valves[t1 as usize] {
                if visited & (1 << i) == 0 {
                    t1 -= min_dist;
                    bound += t1 * flow;
                    if t1 < t2 {
                        (t1, t2) = (t2, t1)
                    }
                    visited |= 1 << i;
                    continue 'next_valve;
                }
            }
            return bound;
        }

        // let mut pressure =
    }
}

fn max_pressure_release(
    start: usize,
    time_left: [u16; 2],
    edges_dist: &[Vec<u16>],
    flows: &[u16],
    best_valves: &[Vec<(usize, u16, u16)>],
) -> u16 {
    let mut seen = HashSet::with_capacity(1024);
    let mut queue = BinaryHeap::with_capacity(1024);

    let init = State {
        visited: 1 << start,
        time_left,
        position: [start, start],
        presure_released: 0,
    };

    queue.push(Priority(u16::MAX, init));

    let mut best = 0;

    while let Some(Priority(upper, state)) = queue.pop() {
        if upper <= best {
            return best;
        }
        if !seen.insert(State {
            presure_released: 0,
            ..state
        }) {
            continue;
        }

        for (index, &edge) in edges_dist[state.position[0]].iter().enumerate() {
            if edge < state.time_left[0] && state.visited & (1 << index) == 0   {
                let time = state.time_left[0] - edge;
                let mut next_state = State {
                    visited: state.visited | 1 << index,
                    time_left: [time, state.time_left[1]],
                    position: [index, state.position[1]],
                    presure_released: state.presure_released + time * flows[index],
                };

                if next_state.time_left[0] < next_state.time_left[1] {
                    next_state.time_left.swap(0, 1);
                    next_state.position.swap(0, 1);
                }

                let upper = next_state.calc_upper(best_valves);

                best = best.max(next_state.presure_released);
                if upper > best{

                    queue.push(Priority(upper, next_state));

                }
            }
        }
    }
    best
}
