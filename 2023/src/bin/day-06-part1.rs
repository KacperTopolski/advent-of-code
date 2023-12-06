use std::io;
use std::io::prelude::*;

fn sol(time: i64, dist: i64) -> i64 {
    dbg!(time, dist);
    (0..=time).map(|x| x * (time - x)).filter(|&x| x > dist).count() as i64
}

fn solve(input: Vec<String>) -> i64 {
    let times: Vec<i64> = input[0].split(' ').map(|x| x.parse::<i64>()).flatten().collect();
    let distances: Vec<i64> = input[1].split(' ').map(|x| x.parse::<i64>()).flatten().collect();

    let ans = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| sol(*t, *d))
        .fold(1i64, |a, b| a * b);

    dbg!(ans)
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
    assert_eq!(ans, 288);
}
