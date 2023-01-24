use std::fs;

fn level_1(input: &str) {
    println!(
        "level_1: {}",
        input
            .split("\n\n")
            .map(|c| {
                c.lines()
                    .map(|l| l.parse::<i32>().unwrap_or_default())
                    .sum::<i32>()
            })
            .max()
            .unwrap()
    );
}

fn level_2(input: &str) {
    let mut supplies = input
        .split("\n\n")
        .map(|c| {
            c.lines()
                .map(|l| l.parse::<i32>().unwrap_or_default())
                .sum()
        })
        .collect::<Vec<i32>>();

    supplies.sort_by(|a, b| b.cmp(a));

    println!("level_2: {}", supplies.iter().take(3).sum::<i32>());
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day01/input.in").expect("file not found");

    level_1(&input);
    level_2(&input);
    println!("time: {:?}", now.elapsed());
}
