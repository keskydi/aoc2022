#![allow(unused)]

use std::{fs, process::id};

type T = usize;

fn level_1(input: &str) -> T {
    let array_size: usize = input.lines().into_iter().next().unwrap().len();

    let table = input.replace(&['\r', '\n'], "");
    let table = table.as_bytes();

    table.iter().enumerate().fold(0, |acc, (index, value)| {
        let x = index % array_size;
        let y = (index - x) / array_size;
        // right
        let right = !table[y * array_size..index].iter().any(|v| v >= value);

        let left = !table[index + 1..(y + 1) * array_size]
            .iter()
            .any(|v| v >= value);

        let top = !table[x..index]
            .iter()
            .step_by(array_size)
            .any(|v| v >= value);

        let bottom = !table[index..]
            .iter()
            .skip(array_size)
            .step_by(array_size)
            .any(|v| v >= value);

        if right | left | top | bottom {
            acc + 1
        } else {
            acc
        }
    })
}

use std::array;

fn level_2(input: &str) -> T {
    let array_size: usize = input.lines().into_iter().next().unwrap().len();

    let table = input.replace(&['\r', '\n'], "");
    let table = table.as_bytes();

    table
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let x = index % array_size;
            let y = (index - x) / array_size;
            // right

            let cl = |(index, continu): (usize, bool), v| -> (usize, bool) {
                if !continu {
                    (index, continu)
                } else if v < value {
                    (index + 1, continu)
                } else {
                    (index + 1, false)
                }
            };

            let right = table[y * array_size..index]
                .iter()
                .rev()
                .fold((0, true), cl)
                .0;

            let left = table[index + 1..(y + 1) * array_size]
                .iter()
                .fold((0, true), cl)
                .0;

            let top = table[x..index]
                .iter()
                .step_by(array_size)
                .rev()
                .fold((0, true), cl)
                .0;

            let bottom = table[index..]
                .iter()
                .skip(array_size)
                .step_by(array_size)
                .fold((0, true), cl)
                .0;

            right * left * top * bottom
        })
        .max()
        .unwrap()
}

#[test]
fn test_level_1() {
    let input = "30373
25512
65332
33549
35390";

    assert_eq!(21, level_1(input))
}

#[test]
fn test_level_2() {
    let input = "30373
25512
65332
33549
35390";

    assert_eq!(8, level_2(input))
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day08/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}
