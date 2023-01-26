use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn level_1(input: &[u8]) -> usize {
    let n = input.iter().position(|&x| x == b'\n').unwrap() as isize;
    let ipos = input.iter().position(|&x| x == b'S').unwrap() as isize;

    let mut queue: VecDeque<(isize, usize)> = VecDeque::from([(ipos, 0)]);
    let mut vst = HashSet::new();

    while let Some((pos, steps)) = queue.pop_front() {
        for (fwd, dpos) in [(false, n + 1), (true, 1), (true, n + 1), (false, 1)] {
            let next = if fwd { pos + dpos } else { pos - dpos };

            if (0..input.len()).any(|v| v == next as usize) && next % (n + 1) != n {
                if input[next as usize] == b'E' && input[pos as usize] == b'z' {
                    return steps + 1;
                } else if (input[next as usize] <= input[pos as usize] + 1
                    || input[pos as usize] == b'S')
                    && !vst.contains(&next)
                    && input[next as usize] >= b'a'
                {
                    vst.insert(next);
                    queue.push_back((next, steps + 1));
                }
            }
        }
    }
    0
}

fn level_2(input: &[u8]) -> usize {
    let n = input.iter().position(|&x| x == b'\n').unwrap() as isize;
    let ipos = input.iter().position(|&x| x == b'E').unwrap() as isize;

    let mut queue: VecDeque<(isize, usize)> = VecDeque::from([(ipos, 0)]);
    let mut vst = HashSet::new();

    while let Some((pos, steps)) = queue.pop_front() {
        for (fwd, dpos) in [(false, n + 1), (true, 1), (true, n + 1), (false, 1)] {
            let next = if fwd { pos + dpos } else { pos - dpos };

            if (0..input.len()).any(|v| v == next as usize) && next % (n + 1) != n {
                let c = if input[pos as usize] == b'E' {
                    b'z' + 1
                } else {
                    input[pos as usize]
                };
                if input[next as usize] == b'a' && input[pos as usize] == b'b' {
                    return steps + 1;
                } else if input[next as usize] >= c - 1
                    && !vst.contains(&next)
                    && input[next as usize] >= b'a'
                {
                    vst.insert(next);
                    queue.push_back((next, steps + 1));
                }
            }
        }
    }
    0
}

// BFS
fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/bin/day12/input.in").expect("file not found");

    println!("level_1: {}", level_1(&input.as_bytes()));
    println!("level_2: {}", level_2(&input.as_bytes()));

    println!("time: {:?}", now.elapsed());
}
