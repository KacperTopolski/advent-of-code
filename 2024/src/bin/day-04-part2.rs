use std::io;
use std::io::prelude::*;
use itertools::{Itertools};

fn score(input: &Vec<Vec<char>>, i: i64, j: i64) -> bool {
    let at = |x: i64, y: i64| {
        input.get(x as usize).map(|v| v.get(y as usize).map(|x| *x)).flatten()
    };
    for dx in vec![-1, 1] {
        let here = (at(i + dx, j + 1), at(i - dx, j - 1));
        // dbg!(i, j, here);
        if here != (Some('M'), Some('S')) && here != (Some('S'), Some('M')) {
            return false
        }
    }

    at(i, j) == Some('A')
}

fn solve(input: Vec<String>) -> i64 {
    let input: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();
    let r = (0..input.len())
        .cartesian_product(0..input[0].len())
        .map(|(i, j)| score(&input, i as i64, j as i64) as i64)
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
    assert_eq!(ans, 9);
}

#[test]
fn test1() {
    let txt = "\
M.S
.A.
S.M
";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 0);
}
