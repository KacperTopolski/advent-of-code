use std::io;
use std::io::prelude::*;
use std::iter::once;

fn backtrack(board: &mut Vec<String>, i: usize, j: usize) -> isize {
    if i + 2 == board.len() && j + 2 == board[0].len() {
        return 0;
    }
    let mut best = -1000000;
    if board[i].as_bytes()[j] == ('#' as u8) {
        return best;
    }
    unsafe {
        board[i].as_mut_vec()[j] = '#' as u8;
    }

    best = best.max(backtrack(board, i + 1, j) + 1);
    best = best.max(backtrack(board, i - 1, j) + 1);
    best = best.max(backtrack(board, i, j + 1) + 1);
    best = best.max(backtrack(board, i, j - 1) + 1);

    unsafe {
        board[i].as_mut_vec()[j] = '.' as u8;
    }

    best
}

fn solve(input: Vec<String>) -> isize {
    let block: String = (0..input[0].len()).map(|_| '#').collect();
    let board: Vec<String> = once(&block).chain(input.iter()).chain(once(&block)).cloned().collect();
    let mut board: Vec<String> = board.iter().map(|s| s.replace(|c| c != '#', ".")).collect();
    drop(input);
    drop(block);

    dbg!(&board);

    backtrack(&mut board, 1, 1)
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
    assert_eq!(ans, 154);
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
