use std::{fs, ops::Range};

fn level_1(input: &str) -> i32 {
    let mut inst: Vec<i32> = vec![];
    let mut l_value = 1;

    for l in input.lines() {
        if l == "noop" {
            inst.push(l_value);
        } else {
            inst.push(l_value);

            l_value += l.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            inst.push(l_value);
        }
    }

    inst.iter()
        .enumerate()
        .map(|(i, &x)| x * (i + 2) as i32)
        .skip(18)
        .step_by(40)
        .sum()
}

fn level_2(input: &str) {
    let mut l_value = 1;

    let mut i = 0;

    for l in input.lines() {
        if l == "noop" {
            display(&mut i, l_value..l_value + 3);
        } else {
            display(&mut i, l_value..l_value + 3);
            display(&mut i, l_value..l_value + 3);

            l_value += l.split_once(" ").unwrap().1.parse::<i32>().unwrap();
        }
    }
    println!()
}

fn display(i: &mut i32, range: Range<i32>) {
    *i = (*i % 40) + 1;
    if i == &1 {
        println!()
    }
    if range.contains(i) {
        print!("█");
    } else {
        print!(".");
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day10/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    print!("level_2 :");
    level_2(&input);

    println!("time: {:?}", now.elapsed());
}
