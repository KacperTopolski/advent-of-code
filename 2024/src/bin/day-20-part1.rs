use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

pub trait Board {
    fn find(&self, ch: char) -> (usize, usize);
    fn dists(&self, i: usize, j: usize) -> Vec<Vec<i64>>;
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

    fn dists(&self, i: usize, j: usize) -> Vec<Vec<i64>> {
        let inf = 1e9 as i64;

        let mut d: Vec<Vec<i64>> = vec![vec![inf; self[0].len()]; self.len()];
        let mut bfs: VecDeque<(usize, usize)> = VecDeque::new();


        d[i][j] = 0;
        bfs.push_back((i, j));

        while let Some((i, j)) = bfs.pop_front() {
            for (x, y) in [(i+1, j), (i-1, j), (i, j-1), (i, j+1)] {
                if self[x][y] == '.' && d[x][y] == inf {
                    d[x][y] = d[i][j] + 1;
                    bfs.push_back((x, y));
                }
            }
        }

        d
    }
}

fn solve(threshold: i64, input: Vec<String>) -> i64 {
    let mut board: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    let (si, sj) = board.find('S');
    let (ei, ej) = board.find('E');

    board[si][sj] = '.';
    board[ei][ej] = '.';

    let from_s = board.dists(si, sj);
    let from_e = board.dists(ei, ej);

    let mut r = 0;

    let (n, m) = (board.len(), board[0].len());

    let normal_path = from_s[ei][ej];

    for i in 0..n {
        for j in 0..m {
            for x in i.saturating_sub(2)..n.min(i+3) {
                for y in j.saturating_sub(2)..m.min(j+3) {
                    if i.abs_diff(x) + j.abs_diff(y) != 2 {
                        continue;
                    }

                    let here = from_s[i][j] + from_e[x][y] + 2;
                    if here + threshold <= normal_path {
                        r += 1;
                    }
                }
            }
        }
    }

    dbg!(r)
}

fn main() {
    solve(100, io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let ans = solve(10, txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 10);
}
