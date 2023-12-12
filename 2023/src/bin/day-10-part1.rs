use std::io;
use std::io::prelude::*;

type C = (usize, usize);

fn find_start(map: &Vec<String>) -> C {
    for (i, row) in map.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            if ch == 'S' {
                return (i, j);
            }
        }
    }
    panic!()
}

fn find_second(map: &Vec<String>, start: C) -> C {
    let (i, j) = start;
    for xy in [(i, j-1), (i, j+1), (i-1, j), (i+1, j)] {
        if neighbour_of(map, xy).contains(&start) {
            return xy;
        }
    }
    panic!()
}

fn neighbour_of(map: &Vec<String>, ij: C) -> Vec<C> {
    let (i, j) = ij;
    match map[i][j..].chars().nth(0).unwrap_or('.') {
        '|' => vec![(i-1, j), (i+1, j)],
        '-' => vec![(i, j-1), (i, j+1)],
        'L' => vec![(i-1, j), (i, j+1)],
        'J' => vec![(i-1, j), (i, j-1)],
        '7' => vec![(i+1, j), (i, j-1)],
        'F' => vec![(i+1, j), (i, j+1)],
        _ => vec![]
    }
}

fn neighbour_of_diff_than(map: &Vec<String>, ij: C, than: C) -> C {
    for st in neighbour_of(map, ij) {
        if st != than {
            return st;
        }
    }
    panic!()
}

fn solve(input: Vec<String>) -> i64 {
    let start = find_start(&input);

    let mut x = start;
    let mut y = find_second(&input, start);

    for i in 0..1_000_000 {
        // dbg!(i, x, y);
        if y == start {
            return dbg!((i + 1) / 2);
        }
        (x, y) = (y, neighbour_of_diff_than(&input, y, x));
    }
    panic!()
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
.....
.S-7.
.|.|.
.L-J.
.....";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 4);
}

#[test]
fn sample_2() {
    let txt = "\
...F7.
..FJ|.
.SJ.L7
.|F--J
.LJ...";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 8);
}
