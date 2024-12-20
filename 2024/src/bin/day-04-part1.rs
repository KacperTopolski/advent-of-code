use std::io;
use std::io::prelude::*;
use itertools::{Itertools};

fn score(input: &Vec<Vec<char>>, i: i64, j: i64) -> i64 {
    let at = |x: i64, y: i64| {
        input.get(x as usize).map(|v| v.get(y as usize)).flatten()
    };
    let dirs: Vec<i64> = vec![-1, 0, 1];
    dirs.iter().cartesian_product(dirs.iter()).map(|(dx, dy)|
        "XMAS".chars().enumerate().map(|(idx, ch)|
            at(i + dx * (idx as i64), j + dy * (idx as i64)) == Some(&ch)
        ).all(|x| x) as i64
    ).sum()
}

fn solve(input: Vec<String>) -> i64 {
    let input: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();
    let r = (0..input.len())
        .cartesian_product(0..input[0].len())
        .map(|(i, j)| score(&input, i as i64, j as i64))
        .sum();
    dbg!(r)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 18);
}
