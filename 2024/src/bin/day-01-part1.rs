use std::io;
use std::io::prelude::*;

fn parse_pair(input: &str) -> (i64, i64) {
    let split: Vec<i64> = input.split_whitespace().map(|x| x.parse::<i64>()).flatten().collect();
    (split[0], split[1])
}

fn solve(input: Vec<String>) -> i64 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.iter().map(|x| parse_pair(x)).unzip();
    left.sort();
    right.sort();
    dbg!(left.iter().zip(right.iter()).map(|(x, y)| (x - y).abs()).sum())
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
3   4
4   3
2   5
1   3
3   9
3   3";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 11);
}
