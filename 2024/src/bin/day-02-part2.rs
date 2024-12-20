use std::io;
use std::io::prelude::*;

fn parse(input: &Vec<String>) -> Vec<Vec<i64>> {
    input.iter().map(
        |x| x.split_whitespace().map(|x| x.parse()).flatten().collect()
    ).collect()
}

fn check_normal(level: &Vec<i64>) -> bool {
    if !level.iter().is_sorted() && !level.iter().rev().is_sorted() {
        return false;
    }
    level.iter().zip(level.iter().skip(1)).map(|(x, y)| {
        let a = (x - y).abs();
        1 <= a && a <= 3
    }).all(|x| x)
}

fn check(level: &Vec<i64>) -> bool {
    for i in 0..=level.len() {
        let mut here = level.clone();
        if i != level.len() {
            here.remove(i);
        }
        if check_normal(&here) {
            return true
        }
    }
    false
}

fn solve(input: Vec<String>) -> i64 {
    let input = parse(&input);
    dbg!(input.iter().filter(|x| check(*x)).count()) as i64
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 4);
}
