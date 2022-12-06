use std::fs;

fn level_1(input: &str) -> isize {
    input
        .lines()
        .filter_map(|l| {
            let len = l.len();
            for char in l.chars().take(len / 2) {
                if l.chars().rev().take(len / 2).any(|c| c == char) {
                    return Some(parse_letter(char));
                }
            }
            None
        })
        .sum::<isize>()
    // unimplemented!();
}

fn parse_letter(c_value: char) -> isize {
    let value = c_value as isize;
    if value >= 97 {
        value - 96
    } else {
        value - 38
    }
}

fn level_2(input: &str) ->isize{
    let lines_1 = input.lines().step_by(3);
    let lines_2 = input.lines().skip(1).step_by(3);
    let lines_3 = input.lines().skip(2).step_by(3);

    lines_1.zip(lines_2.zip(lines_3))
        .filter_map(|(a,(b,c))| {
            for char in a.chars() {
                if b.contains(char) && c.contains(char) {
                    return Some(parse_letter(char));
                }
            }
            None
        })
        .sum::<isize>()
}

#[test]
fn test_level_1() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(157, level_1(input))
}

#[test]
fn test_level_2() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(70, level_2(input))
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day03/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));
    println!("time: {:?}", now.elapsed());
}
