use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn parse_pair(input: &str) -> (i64, i64) {
    let split: Vec<i64> = input.split_whitespace().map(|x| x.parse::<i64>()).flatten().collect();
    (split[0], split[1])
}

fn frequency_map(input: Vec<i64>) -> HashMap<i64, i64> {
    let mut mp: HashMap<i64, i64> = input.iter().map(|x| (*x, 0i64)).collect();
    for i in input {
        *mp.get_mut(&i).unwrap() += 1
    }
    mp
}

fn solve(input: Vec<String>) -> i64 {
    let (left, right): (Vec<_>, Vec<_>) = input.iter().map(|x| parse_pair(x)).unzip();
    let right = frequency_map(right);

    dbg!(left.iter().map(|x| x * *right.get(x).or(Some(&0)).unwrap()).sum())
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
    assert_eq!(ans, 31);
}
