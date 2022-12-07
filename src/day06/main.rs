use std::{fs};

type T = usize;

fn level_1(input: &str) -> T {
    // input
    input.as_bytes().windows(4).position(|slice|{
        !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
    }).unwrap() + 4
}

fn level_2(input: &str) -> T {
    input.as_bytes().windows(14).position(|slice|{
        !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
    }).unwrap() + 14
}

#[test]
fn test_level_1(){
    assert_eq!(level_1("bvwbjplbgvbhsrlpgdmjqwftvncz"),5);
    assert_eq!(level_1("nppdvjthqldpwncqszvftbrmjlhg"),6);
    assert_eq!(level_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),10);
    assert_eq!(level_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),11);
}

#[test]
fn test_level_2(){
    assert_eq!(level_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),19);
    assert_eq!(level_2("bvwbjplbgvbhsrlpgdmjqwftvncz"),23);
    assert_eq!(level_2("nppdvjthqldpwncqszvftbrmjlhg"),23);
    assert_eq!(level_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),29);
    assert_eq!(level_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),26);
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day06/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}
