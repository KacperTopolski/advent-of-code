use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::iter::once;
use itertools::Itertools;

fn solve(input: Vec<String>) -> i64 {
    let board: Vec<Vec<u32>> = input.iter().map(|s| s.chars().map(|c| c.to_digit(10)).flatten().collect()).collect();

    let advance = |s: HashSet<(i64, i64)>| {
        if s.is_empty() {
            return s
        }
        let (ai, aj) = s.iter().last().unwrap();
        let should = 1 + board[*ai as usize][*aj as usize];

        (0..board.len()).cartesian_product(0..board[0].len())
            .filter(|(x, y)| board[*x][*y] == should)
            .map(|(x, y)| (x as i64, y as i64))
            .filter(|(x, y)| {
                s.contains(&(*x, y - 1)) || s.contains(&(*x, y + 1)) || s.contains(&(x - 1, *y)) || s.contains(&(x + 1, *y))
            })
            .collect()
    };

    let r = (0..board.len()).cartesian_product(0..board[0].len())
        .filter(|(x, y)| board[*x][*y] == 0)
        .map(|(x, y)| (x as i64, y as i64))
        .map(|(x, y)| {
            let mut hs: HashSet<(i64, i64)> = once((x, y)).collect();
            for _ in 0..9 {
                hs = advance(hs);
            }
            hs.len() as i64
        }).sum();

    dbg!(r)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 36);
}
