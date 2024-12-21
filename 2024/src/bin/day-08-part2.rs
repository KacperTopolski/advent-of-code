use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn inline(i: i64, j: i64, x: i64, y: i64, s: i64, t: i64) -> bool {
    (i - x) * (t - y) == (s - x) * (j - y)
}

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
                    for s in 0..board.len() {
                        for t in 0..board[0].len() {
                            if inline(i as i64, j as i64, x as i64, y as i64, s as i64, t as i64) {
                                hs.insert((s, t));
                            }
                        }
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
    assert_eq!(ans, 34);
}
