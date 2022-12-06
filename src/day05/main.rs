use std::fs;

fn level_1(input: &str) -> isize {
    let (stack, moves) = input.split_once("\r\n\r\n").unwrap();

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

    let moves = moves.splitn(6, ' ');
    let _moves = moves.map(|mv| {
        let mut c_moves = mv.splitn(6, ' ').skip(1).step_by(2);

        let (Some(nb),Some(start),Some(end)) = (c_moves.next(),c_moves.next(),c_moves.next()) else{
            panic!("Can't parse move: '{mv}'");
        };
        (nb, start, end)
    });

    0
}

fn level_2(input: &str) -> isize {
    unimplemented!();
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day05/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));
    println!("time: {:?}", now.elapsed());
}
