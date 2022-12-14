#![allow(warnings)]

use std::fs;

struct Monkeys {
    monkeys: Vec<(fn(u64) -> (), Vec<u64>)>,
}

fn t() {
    let size = 8;
    let items: Vec<Vec<u64>> = Vec::with_capacity(size);
}

enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

impl Op {
    fn operate(&self, value: u64) -> u64 {
        match self {
            Op::Add(v) => value + v,
            Op::Mul(v) => value * v,
            Op::Square => value * value,
        }
    }
}

fn level_1(input: &str) -> u64 {
// fn level_1(input: &str, count: u64) -> u64 {
    let mut items: Vec<Vec<u64>> = vec![];
    let mut stress: Vec<u64> = vec![];

    let monkeys: Vec<(Op, u64, u64, u64)> = input
        .split("\n\n")
        .map(|s| {
            let (m, v) = parse_monkey(s);
            items.push(v);
            stress.push(0);
            m
        })
        .collect();

    let mut iter = std::iter::from_fn(move || {

        // let mut nxt: Vec<Vec<u64>> = vec![vec![];items.len()];

        for (idx,(op,div,t,f)) in monkeys.iter().enumerate() {

            while let Some(mut item) = items.get_mut(idx).unwrap().pop(){
                *stress.get_mut(idx).unwrap() += 1;
                item = op.operate(item).div_euclid(3);

                if item % div == 0 {
                    items.get_mut(*t as usize).unwrap().push(item);
                }else{
                    items.get_mut(*f as usize).unwrap().push(item);
                }
            }
        }

        Some(stress.clone())
    });

    let mut lv_1 = iter.nth(19).unwrap();
    lv_1.sort();
    lv_1.reverse();



    lv_1[0]*lv_1[1]
}

fn level_2(input: &str) -> u128 {
    // fn level_1(input: &str, count: u64) -> u64 {
        let mut items: Vec<Vec<u64>> = vec![];
        let mut stress: Vec<u128> = vec![];

        let mut lcm = 1;
    
        let monkeys: Vec<(Op, u64, u64, u64)> = input
            .split("\n\n")
            .map(|s| {
                let (m, v) = parse_monkey(s);
                items.push(v);
                stress.push(0);
                lcm *= m.1;
                m
            })
            .collect();
    
        dbg!(lcm);
        let mut iter = std::iter::from_fn(move || {
    
            // let mut nxt: Vec<Vec<u64>> = vec![vec![];items.len()];
    
            for (idx,(op,div,t,f)) in monkeys.iter().enumerate() {
    
                while let Some(mut item) = items.get_mut(idx).unwrap().pop(){
                    *stress.get_mut(idx).unwrap() += 1;
                    item = op.operate(item) % lcm;
    
                    if item % div == 0 {
                        items.get_mut(*t as usize).unwrap().push(item);
                    }else{
                        items.get_mut(*f as usize).unwrap().push(item);
                    }
                }
            }
    
            Some(stress.clone())
        });
    
        let mut lv_1 = iter.nth(9999).unwrap();
        lv_1.sort();
        lv_1.reverse();
    
    
    
        lv_1[0]*lv_1[1]
    }

fn parse_monkey(s: &str) -> ((Op, u64, u64, u64), Vec<u64>) {
    let mut inst = s.lines().skip(1);
    // inst.skip(1);
    let (Some(items),Some(op),Some(test),Some(t_target),Some(f_target)) = (inst.next(),inst.next(),inst.next(),inst.next(), inst.next()) else {
        panic!("Can't parse monkey: '{s}'");
    };

    let op = if let Ok(value) = op[24..].trim().parse::<u64>() {
        match op.chars().nth(23) {
            Some('*') => Op::Mul(value),
            Some('+') => Op::Add(value),
            Some(v) => panic!("unknown operation : {v} "),
            _ => panic!("unknown operation : None "),
        }
    } else {
        Op::Square
    };

    let items: Vec<u64> = items[17..].trim()
        .split(",")
        .map(|x| {
            x.trim().parse::<u64>().unwrap()
        })
        .collect();

    (
        (
            op,
            test[20..].trim().parse::<u64>().unwrap(),
            t_target[28..].trim().parse::<u64>().unwrap(),
            f_target[29..].trim().parse::<u64>().unwrap(),
        ),
        items,
    )
}

fn main() {
    let now = std::time::Instant::now();
    let input = fs::read_to_string("src/day11/input.in").expect("file not found");


    println!("level_1: {}", level_1(&input));
    println!("level_2: {}", level_2(&input));

    println!("time: {:?}", now.elapsed());
}
