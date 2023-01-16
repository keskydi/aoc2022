use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    error::Error,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

// return sum of pressure release

fn floyd_warshall(graph: &[(&str, u8, Vec<&str>)]) -> Vec<Vec<u8>> {
    let num_of_nodes = graph.len();

    let valve_name_to_idx: HashMap<&str, _> = graph
        .iter()
        .enumerate()
        .map(|(i, &(id, ..))| (id, i))
        .collect();

    // |V|x|V| matrix
    let mut dist = vec![vec![u8::MAX; num_of_nodes]; num_of_nodes];

    for (i, (_, _, edges)) in graph.iter().enumerate() {
        for edge in edges {
            let y = valve_name_to_idx[edge];
            dist[i][y] = 1;
        }
        dist[i][i] = 0;
    }
    // init distances of paths with no intermediate nodes

    for k in 0..num_of_nodes {
        for i in 0..num_of_nodes {
            for j in 0..num_of_nodes {
                let (result, overflow) = dist[i][k].overflowing_add(dist[k][j]);
                if !overflow && dist[i][j] > result {
                    dist[i][j] = result;
                }
            }
        }
    }

    dist
}

fn parse_input(s: &str) -> IResult<&str, Vec<(&str, u8, Vec<&str>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            separated_pair(
                preceded(tag::<_, _, Error<_>>("Valve "), alpha1),
                tag(" has flow rate="),
                complete::u8,
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
fn main() {
    let data = fs::read_to_string("src/day16/input.in").expect("file not found");

    // let (_, mut graph) = parse_input(&input).unwrap();
    let (_, rows) = parse_input(&data).unwrap();
    let shortest_path_lengths_uncompressed = floyd_warshall(&rows);

    let interesting_values: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter_map(|(id, &(name, flow, _))| {
            if name == "AA" || flow > 0 {
                Some(id)
            } else {
                None
            }
        })
        .collect();

    let flow_rate = interesting_values
        .iter()
        .map(|&i| rows[i].1)
        .collect::<Vec<u8>>();

    let shortest_path_lengths: Vec<Vec<u8>> = interesting_values
        .iter()
        .map(|&i| {
            interesting_values
                .iter()
                .map(|&y| shortest_path_lengths_uncompressed[i][y])
                .collect()
        })
        .collect();

    let starting_node = interesting_values
        .iter()
        .position(|&i| rows[i].0 == "AA")
        .unwrap();

    let now = std::time::Instant::now();

    println!(
        "level_1: {}",
        bfs_part1(&flow_rate, &shortest_path_lengths, starting_node, 0, 31)
    );

    println!(
        "level_2: {}",
        bfs_part2(
            &flow_rate,
            &shortest_path_lengths,
            starting_node,
            starting_node,
            0,
            27,
            27
        )
    );

    println!("Time: {:?}", now.elapsed());
}

fn bfs_part1(
    flow_rates: &Vec<u8>,
    shortest_path_lengths: &Vec<Vec<u8>>,
    indice: usize,
    mut vst: usize,
    time_left: u8,
) -> usize {
    vst |= 1 << indice;

    let pressure_release = flow_rates[indice] as usize * (time_left - 1) as usize;

    let result = shortest_path_lengths[indice]
        .iter()
        .enumerate()
        .map(|(i, dist)| {
            if dist + 1 < time_left && vst & 1 << i == 0 {
                bfs_part1(
                    flow_rates,
                    shortest_path_lengths,
                    i,
                    vst,
                    time_left - dist - 1,
                )
            } else {
                0
            }
        })
        .max()
        .unwrap()
        + pressure_release;

    result
}

fn bfs_part2(
    flow_rates: &Vec<u8>,
    shortest_path_lengths: &Vec<Vec<u8>>,
    indice1: usize,
    indice2: usize,
    mut vst: usize,
    time_left1: u8,
    time_left2: u8,
) -> usize {
    vst |= 1 << indice1 | 1 << indice2;

    let pressure_release = flow_rates[indice1] as usize * (time_left1 - 1) as usize
        + flow_rates[indice2] as usize * (time_left2 - 1) as usize;

    shortest_path_lengths[indice1]
        .iter()
        .enumerate()
        .map(|(i1, dist1)| {
            if dist1 + 1 < time_left1 && vst & 1 << i1 == 0 {
                shortest_path_lengths[indice2]
                    .iter()
                    .enumerate()
                    .map(|(i2, dist2)| {
                        if dist2 + 1 < time_left2 && vst & 1 << i2 == 0 && i1 != i2 {
                            bfs_part2(
                                flow_rates,
                                shortest_path_lengths,
                                i1,
                                i2,
                                vst,
                                time_left1 - dist1 - 1,
                                time_left2 - dist2 - 1,
                            )
                        } else {
                            0
                        }
                    })
                    .max()
                    .unwrap()
            } else {
                0
            }
        })
        .max()
        .unwrap()
        + pressure_release
}
