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
    let input = fs::read_to_string("src/dayXX/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}
