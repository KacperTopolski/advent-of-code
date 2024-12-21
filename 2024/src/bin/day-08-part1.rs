use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn solve(input: Vec<String>) -> i64 {
    let board: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();
    let mut hs: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            for x in 0..board.len() {
                for y in 0..board[0].len() {
                    if board[i][j] == '.' || board[i][j] != board[x][y] || (i, j) == (x, y) {
                        continue
                    }
                    if 2 * i >= x && 2 * i - x < board.len() && 2 * j >= y && 2 * j - y < board[0].len() {
                        hs.insert((2 * i - x, 2 * j - y));
                    }
                }
            }
        }
    }

    dbg!(hs.len()) as i64
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 14);
}
