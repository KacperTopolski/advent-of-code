use std::io;
use std::io::prelude::*;

fn get_next(input: Vec<i64>) -> i64 {
    if input.iter().all(|&x| x == 0) {
        return 0;
    }

    let diff: Vec<i64> = (1..input.len()).map(|idx| input[idx] - input[idx - 1]).collect();

    input.first().unwrap() - get_next(diff)
}

fn solve_test(input: &String) -> i64 {
    let input: Vec<i64> = input.split(' ').map(|x| x.parse::<i64>()).flatten().collect();
    get_next(input)
}

fn solve(input: Vec<String>) -> i64 {
    dbg!(input.iter().map(solve_test).sum())
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 2);
}
