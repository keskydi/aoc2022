use std::fs;

fn level_1(input: &str) -> isize {
    input
        .lines()
        .fold(0, |acc,l|{
            let mut splits = l.splitn(4, ['-',',']).map(|x|x.parse::<isize>().unwrap());
            let (Some(start1),Some(end1),Some(start2),Some(end2)) = (splits.next(),splits.next(),splits.next(),splits.next()) else{
                panic!("Can't split line in pair: '{l}'");
            };
            if (start1 <= start2 && end2 <= end1) || (start2 <= start1 && end1 <= end2){
                acc + 1
            }else{
                acc
            }
        })
}

fn level_2(input: &str) -> isize {
    input
        .lines()
        .fold(0, |acc,l|{
            let mut splits = l.splitn(4, ['-',',']).map(|x|x.parse::<isize>().unwrap());
            let (Some(start1),Some(end1),Some(start2),Some(end2)) = (splits.next(),splits.next(),splits.next(),splits.next()) else{
                panic!("Can't split line in pair: '{l}'");
            };
            if start1 <= end2 && start2 <= end1 {
                acc + 1
            }else{
                acc
            }
        })
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/bin/day04/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));
    println!("time: {:?}", now.elapsed());
}
