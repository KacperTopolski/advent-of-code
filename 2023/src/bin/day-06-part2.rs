use std::io;
use std::io::prelude::*;

fn solve(input: Vec<String>) -> usize {
    let time = input[0].chars().filter(|x| x.is_digit(10)).collect::<String>().parse::<i64>().unwrap();
    let dist = input[1].chars().filter(|x| x.is_digit(10)).collect::<String>().parse::<i64>().unwrap();

    dbg!((0..=time).map(|x| x * (time - x)).filter(|&x| x > dist).count())
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
Time:      7  15   30
Distance:  9  40  200";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 71503);
}
