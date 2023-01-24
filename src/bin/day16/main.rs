use aoc2022::Priority;
// use hashbrown::{HashMap, HashSet};
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
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, fs};

fn parse_input(s: &str) -> IResult<&str, Vec<(&str, u32, Vec<&str>)>> {
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
        .map(|((ident, rate), edges)| (ident, rate, edges)),
    )(s)
}

fn floyd_warshall(graph: &[(&str, u32, Vec<&str>)]) -> Vec<Vec<u32>> {
    let map = graph
        .iter()
        .enumerate()
        .map(|(i, &(id, _, _))| (id, i))
        .collect::<HashMap<&str, _>>();

    let n = graph.len();

    let mut dist = vec![vec![u32::MAX; n]; n];

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
    let data = fs::read_to_string("src/bin/day16/input.in").expect("file not found");

    let (_, rows) = parse_input(&data).unwrap();

    let dist = floyd_warshall(&rows);

    let useful_rows = rows
        .iter()
        .enumerate()
        .filter_map(|(i, &(id, flow, _))| (id == "AA" || flow > 0).then(|| i))
        .collect::<Vec<_>>();

    let flow_rates: Vec<_> = useful_rows.iter().map(|&i| rows[i as usize].1).collect();

    let filtered_dist: Vec<Vec<_>> = useful_rows
        .iter()
        .map(|&x| {
            useful_rows
                .iter()
                .map(|&y| dist[x as usize][y as usize] + 1)
                .collect()
        })
        .collect();

    let starting_node = useful_rows
        .iter()
        .position(|&i| rows[i as usize].0 == "AA")
        .unwrap() as u16;

    let best_valves: Vec<Vec<_>> = (0..=30)
        .map(|t| {
            flow_rates
                .iter()
                .enumerate()
                .filter_map(|(i, &flow)| {
                    let min_dist = *filtered_dist[i].iter().filter(|&&v| v > 1).min().unwrap();
                    (t > min_dist).then(|| (i, min_dist, flow))
                })
                .sorted_by_key(|&(_, min_dist, flow)| Reverse(flow * (t - min_dist)))
                .collect()
        })
        .collect();

    let a = max_pressure_release(
        starting_node,
        &filtered_dist,
        &flow_rates,
        &best_valves,
        [30, 0],
    );
    println!("Part 1: {:?}", a);

    let b = max_pressure_release(
        starting_node,
        &filtered_dist,
        &flow_rates,
        &best_valves,
        [26, 26],
    );
    println!("Part 2: {:?}", b);

    println!("Time: {:?}", now.elapsed());
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pressure: u32,
    opened: u16,
    pos: [u16; 2],
    time: [u32; 2],
}

impl State {
    fn upper_bound(&self, best_valves: &[Vec<(usize, u32, u32)>]) -> u32 {
        let [mut max_t, mut min_t] = self.time;
        let mut opened = self.opened;
        let mut bound = self.pressure;

        'next_valve: loop {
            for (i, min_dist, f) in &best_valves[max_t as usize] {
                if opened & (1 << i) == 0 {
                    max_t -= min_dist;
                    bound += f * max_t as u32;
                    if max_t < min_t {
                        (min_t, max_t) = (max_t, min_t);
                    }
                    opened |= 1 << i;
                    continue 'next_valve;
                }
            }
            return bound;
        }
    }
}

fn max_pressure_release(
    start: u16,
    edges: &[Vec<u32>],
    flows: &[u32],
    best_valves: &[Vec<(usize, u32, u32)>],
    time: [u32; 2],
) -> u32 {
    let init = State {
        pressure: 0,
        opened: 1 << start,
        pos: [start, start],
        time,
    };

    let mut seen = HashSet::new();
    let mut best = 0;
    let mut paths = BinaryHeap::new();
    paths.push(Priority(u32::MAX, init));

    while let Some(Priority(upper, cur)) = paths.pop() {
        if upper <= best {
            return best;
        }

        if !seen.insert(State { pressure: 0, ..cur }) {
            continue;
        }

        for (next, edge_len) in edges[cur.pos[0] as usize].iter().enumerate() {
            if cur.time[0] > *edge_len && cur.opened & (1 << next) == 0 {
                let new_time = cur.time[0] - edge_len;
                let mut next_state = State {
                    pressure: cur.pressure + flows[next ] * new_time,
                    opened: cur.opened | (1 << next),
                    pos: [next as u16, cur.pos[1]],
                    time: [new_time, cur.time[1]],
                };

                if next_state.time[0] < next_state.time[1] {
                    next_state.pos.swap(0, 1);
                    next_state.time.swap(0, 1);
                }

                best = best.max(next_state.pressure);
                let upper = next_state.upper_bound(best_valves);
                if upper > best {
                    paths.push(Priority(upper, next_state));
                }
            }
        }
    }
    best
}