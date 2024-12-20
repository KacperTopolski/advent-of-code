use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

pub trait Board {
    fn find(&self, ch: char) -> (usize, usize);
    fn at(&self, i: usize, j: usize) -> Option<char>;
    fn ati(&self, i: i64, j: i64) -> Option<char> {
        if let (Ok(i), Ok(j)) = (usize::try_from(i), usize::try_from(j)) {
            self.at(i, j)
        }
        else {
            None
        }
    }
    fn ati_p(&self, (i, j): (i64, i64)) -> Option<char> {
        self.ati(i, j)
    }
}

impl Board for Vec<Vec<char>> {
    fn find(&self, ch: char) -> (usize, usize) {
        for i in 0..self.len() {
            for j in 0..self[i].len() {
                if self[i][j] == ch {
                    return (i, j);
                }
            }
        }
        panic!()
    }
    fn at(&self, i: usize, j: usize) -> Option<char> {
        self.get(i).and_then(|v| v.get(j)).cloned()
    }
}

fn add((a, b): (i64, i64), (c, d): (i64, i64)) -> (i64, i64) {
    (a + c, b + d)
}

fn loops(board: &Vec<Vec<char>>) -> bool {
    let (i, j) = board.find('^');
    let mut pos = (i as i64, j as i64);

    let dirs: Vec<(i64, i64)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;

    let mut vis = HashSet::new();

    while let Some(_) = board.ati_p(pos) {
        if !vis.insert((pos, dir)) {
            return true
        }

        while board.ati_p(add(pos, dirs[dir])) == Some('#') {
            dir = (dir + 1) % 4;
        }

        pos = add(pos, dirs[dir]);
    }

    false
}

fn solve(input: Vec<String>) -> i64 {
    let mut board: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();
    let mut r = 0;

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board.at(i, j) != Some('.') {
                continue
            }
            board[i][j] = '#';
            if loops(&board) {
                r += 1;
            }
            board[i][j] = '.';
        }
    }

    dbg!(r)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 6);
}
