use std::fs;

type R = String;

fn level_1(input: &str) -> R {
    unimplemented!()
}

fn level_2(input: &str) -> R {
    unimplemented!()
}

fn main() {
    let now = std::time::Instant::now();
    // let input = fs::read_to_string("src/day17/input.in").expect("file not found");

    // println!("level_1: {}", level_1(&input));
    // println!("level_2: {}", level_2(&input));
    let mut best_per_visited = vec![0; u16::MAX as usize];
    dbg!(best_per_visited.len());

    println!("time: {:?}", now.elapsed());
}
