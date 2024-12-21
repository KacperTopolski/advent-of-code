use std::io;
use std::io::prelude::*;
use itertools::Itertools;

fn solve(input: Vec<String>) -> i64 {
    let input: &str = &input[0];

    let mut space: Vec<i64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(id, digit)| vec![if id % 2 == 0 { (id / 2) as i64 } else { -1 }; digit as usize])
        .collect();

    while let Some((i, _)) = space.iter().find_position(|x| **x == -1) {
        if space.last() == Some(&-1) {
            space.pop();
            continue;
        }
        space[i] = space.pop().unwrap()
    }

    let r = space.iter().enumerate().map(|(x, y)| x as i64 * y).sum();
    dbg!(r)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "2333133121414131402";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 1928);
}
