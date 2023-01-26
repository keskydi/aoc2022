use std::collections::BTreeSet;
use std::fs;

fn follow(input: &str, lenght: usize) -> usize {
    let mut lpos: BTreeSet<(i32, i32)> = BTreeSet::new();
    lpos.insert((0, 0));

    let mut line = vec![(0, 0); lenght];

    for l in input.lines() {
        let (dir, steps): (&str, usize) = {
            let (d, step) = l.trim().split_once(" ").unwrap();
            (d, step.parse::<usize>().unwrap())
        };

        for _ in 0..steps {
            let hd = line.get_mut(0).unwrap();
            match dir {
                "L" => hd.0 -= 1,
                "R" => hd.0 += 1,
                "U" => hd.1 -= 1,
                "D" => hd.1 += 1,
                _ => panic!("not allowed!"),
            }

            for i in 1..lenght {
                let hd = line.get(i - 1).unwrap().clone();
                let tl: &mut (i32, i32) = line.get_mut(i).unwrap();

                match (hd.0 - tl.0, hd.1 - tl.1) {
                    (x, y) if x.abs() < 2 && y.abs() < 2 => {}
                    (x, 0) => tl.0 += x.is_positive() as i32 * 2 - 1,
                    (0, y) => tl.1 += y.is_positive() as i32 * 2 - 1,
                    (x, y) => {
                        tl.0 += x.is_positive() as i32 * 2 - 1;
                        tl.1 += y.is_positive() as i32 * 2 - 1;
                    }
                }
            }
            lpos.insert(*line.get(lenght - 1).unwrap());
        }
    }
    lpos.len()
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/bin/day09/input.in").expect("file not found");

    println!("level_1: {}", follow(&input, 2));
    println!("level_2: {}", follow(&input, 10));

    println!("time: {:?}", now.elapsed());
}

#[test]
fn test_follow() {
    let input = "R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2";

    assert_eq!(follow(&input, 2), 13);

    let input = "R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20";
    assert_eq!(follow(&input, 10), 36);
}
