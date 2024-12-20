use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::iter::once;

fn check_line(line: &str) -> Option<i64> {
    let (goal, nums) = line.split_once(":").unwrap();

    let goal: i64 = goal.parse().unwrap();
    let nums: Vec<i64> = nums.split_whitespace().map(|s| s.parse()).flatten().collect();

    let mut can_do: HashSet<i64> = once(nums[0]).collect();

    for &i in nums.iter().skip(1) {
        let mut b = 10;
        while b <= i {
            b *= 10;
        }
        can_do = can_do.iter().flat_map(|x| vec![x * i, x + i, x * b + i]).collect()
    }

    if can_do.contains(&goal) { Some(goal) } else { None }
}

fn solve(input: Vec<String>) -> i64 {
    dbg!(input.iter().filter_map(|x| check_line(x)).sum())
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 11387);
}
