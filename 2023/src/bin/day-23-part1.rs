use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::iter::once;

fn needed(board: &Vec<String>, i: usize, j: usize) -> usize {
    assert_ne!(board[i][j..].chars().nth(0).unwrap(), '#');

    let left = board[i][j-1..].chars().nth(0).unwrap();
    let right = board[i][j+1..].chars().nth(0).unwrap();
    let top = board[i-1][j..].chars().nth(0).unwrap();
    let bottom = board[i+1][j..].chars().nth(0).unwrap();

    let input =
        (if left == '>' {1} else {0}) +
            (if right == '<' {1} else {0}) +
            (if top == 'v' {1} else {0}) +
            (if bottom == '^' {1} else {0});
    let output =
        (if left == '<' {1} else {0}) +
            (if right == '>' {1} else {0}) +
            (if top == '^' {1} else {0}) +
            (if bottom == 'v' {1} else {0});
    let normal =
        (if left == '.' {1} else {0}) +
            (if right == '.' {1} else {0}) +
            (if top == '.' {1} else {0}) +
            (if bottom == '.' {1} else {0});

    (if output > 0 {input + normal} else {input + normal - 1}).max(1)
}

fn out_neighbours(board: &Vec<String>, i: usize, j: usize) -> Vec<(usize, usize)> {
    assert_ne!(board[i][j..].chars().nth(0).unwrap(), '#');

    match board[i][j..].chars().nth(0).unwrap() {
        '.' => vec![(i-1, j), (i+1, j), (i, j-1), (i, j+1)],
        '^' => vec![(i-1, j)],
        'v' => vec![(i+1, j)],
        '<' => vec![(i, j-1)],
        '>' => vec![(i, j+1)],
        _ => panic!()
    }
        .iter()
        .cloned()
        .filter(|(x, y)| board[*x][*y..].chars().nth(0).unwrap() != '#')
        .collect()
}

fn solve(input: Vec<String>) -> usize {
    let block: String = (0..input[0].len()).map(|_| '#').collect();
    let board: Vec<String> = once(&block).chain(input.iter()).chain(once(&block)).cloned().collect();
    drop(block);

    let mut dq: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut seen = vec![vec![0; board[0].len()]; board.len()];
    dq.push_back((0, 1, 1));

    while let Some((dist, i, j)) = dq.pop_front() {
        dbg!(dist, i, j);
        for (x, y) in out_neighbours(&board, i, j) {
            seen[x][y] += 1;
            if seen[x][y] == needed(&board, x, y) {
                dq.push_back((dist + 1, x, y));
            }
        }
        if i + 2 == board.len() && j + 2 == board[0].len() {
            return dist;
        }
    }

    panic!()
}

fn main() {
    let ans = solve(io::stdin().lock().lines().flatten().collect());
    dbg!(ans);
}

#[test]
fn sample() {
    let txt = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    dbg!(ans);
    assert_eq!(ans, 94);
}

#[test]
fn mine() {
    let txt = "\
#.#
#.#";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    dbg!(ans);
    assert_eq!(ans, 1);
}
