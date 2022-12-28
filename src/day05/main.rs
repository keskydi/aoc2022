use std::fs;

fn level_1(input: &str) -> String {
    let (stack, moves) = input.split_once("\n\n").unwrap();

    let mut c_stack: Vec<Vec<char>> = vec![];

    stack.lines().rev().skip(1).for_each(|line| {
        for (index, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                if let Some(collum) = c_stack.get_mut(index) {
                    collum.push(c)
                } else {
                    c_stack.push(vec![c])
                }
            }
        }
    });

    let moves = moves.lines().map(|mv| {
        let mut c_moves = mv.splitn(6, ' ').skip(1).step_by(2);

        let (Some(nb),Some(start),Some(end)) = (c_moves.next(),c_moves.next(),c_moves.next()) else{
            panic!("Can't parse move: '{mv}'");
        };
        (
            nb.parse::<usize>().unwrap(),
            start.parse::<usize>().unwrap(),
            end.parse::<usize>().unwrap(),
        )
    });

    moves.for_each(|(nb, start, end)| {
        for _ in 0..nb {
            let value = c_stack.get_mut(start - 1).unwrap().pop().unwrap();
            c_stack.get_mut(end - 1).unwrap().push(value);
        }
    });

    c_stack
        .iter()
        .map(|s| {
            let c = s.last().unwrap().to_owned();
            c.to_string()
        })
        .collect::<Vec<String>>()
        .join("")
}

fn level_2(input: &str) -> String {
    let (stack, moves) = input.split_once("\n\n").unwrap();

    let mut c_stack: Vec<Vec<char>> = vec![];

    stack.lines().rev().skip(1).for_each(|line| {
        for (index, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                if let Some(collum) = c_stack.get_mut(index) {
                    collum.push(c)
                } else {
                    c_stack.push(vec![c])
                }
            }
        }
    });

    let moves = moves.lines().map(|mv| {
        let mut c_moves = mv.splitn(6, ' ').skip(1).step_by(2);

        let (Some(nb),Some(start),Some(end)) = (c_moves.next(),c_moves.next(),c_moves.next()) else{
            panic!("Can't parse move: '{mv}'");
        };
        (
            nb.parse::<usize>().unwrap(),
            start.parse::<usize>().unwrap(),
            end.parse::<usize>().unwrap(),
        )
    });

    moves.for_each(|(nb, start, end)| {
        let mut list = vec![];
        for _ in 0..nb {
            let value = c_stack.get_mut(start - 1).unwrap().pop().unwrap();
            list.push(value)
        }
        list.iter()
            .rev()
            .for_each(|&value| c_stack.get_mut(end - 1).unwrap().push(value))
    });

    c_stack
        .iter()
        .map(|s| {
            let c = s.last().unwrap().to_owned();
            c.to_string()
        })
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn test_level_1() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!("CMZ", level_1(input))
}

#[test]
fn test_level_2() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!("MCD", level_2(input))
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day05/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));
    println!("time: {:?}", now.elapsed());
}
