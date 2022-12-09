use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    
    let iter = vec![1,2,3,4,5,6,7,8,9,10];

    dbg!(iter[0..0].iter().step_by(3).collect::<Vec<_>>());
    dbg!(iter[0..8].iter().rev().step_by(3).collect::<Vec<_>>());
    dbg!(iter[0..8].iter().step_by(3).rev().collect::<Vec<_>>());
    dbg!(iter[0..8].iter().rev().collect::<Vec<_>>());

}