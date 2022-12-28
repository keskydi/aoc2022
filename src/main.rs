#![allow(unused)]
use std::cmp::Ordering;

fn main() {
    let r = vec![2, 3, 4];
    let l = vec![4];
    assert_eq!(r.cmp(&l), Ordering::Less);

    assert_eq!(3.cmp(&5), Ordering::Less);

    let r = vec![7, 7, 7, 7];
    let l = vec![7, 7, 7];
    assert_ne!(r.cmp(&l), Ordering::Less);

    let r = vec![5, 6, 7];
    let l = vec![5, 6, 0];
    assert_ne!(r.cmp(&l), Ordering::Less);
}
