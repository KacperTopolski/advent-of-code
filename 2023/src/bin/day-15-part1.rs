use std::io;
use std::io::prelude::*;

fn hash(to_hash: &str) -> usize {
    let mut h = 0;
    for ch in to_hash.chars() {
        h = (h + ch as usize) * 17 % 256;
    }
    h
}

fn solve(input: Vec<String>) -> usize {
    dbg!(input[0].split(',').map(hash).sum())
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 1320);
}
