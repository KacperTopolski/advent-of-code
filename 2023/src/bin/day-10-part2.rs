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

fn neighbour_by_type(ch: char, ij: C) -> Vec<C> {
    let (i, j) = ij;
    match ch {
        '|' => vec![(i-1, j), (i+1, j)],
        '-' => vec![(i, j-1), (i, j+1)],
        'L' => vec![(i-1, j), (i, j+1)],
        'J' => vec![(i-1, j), (i, j-1)],
        '7' => vec![(i+1, j), (i, j-1)],
        'F' => vec![(i+1, j), (i, j+1)],
        _ => vec![]
    }
}

fn neighbour_of(map: &Vec<String>, ij: C) -> Vec<C> {
    neighbour_by_type(map[ij.0][ij.1..].chars().nth(0).unwrap_or('.'), ij)
}

fn neighbour_of_diff_than(map: &Vec<String>, ij: C, than: C) -> C {
    for st in neighbour_of(map, ij) {
        if st != than {
            return st;
        }
    }
    panic!()
}

fn solve(mut input: Vec<String>) -> i64 {
    let start = find_start(&input);

    let mut x = start;
    let mut y = find_second(&input, start);

    let mut area = 0;
    let mut loop_part = vec![vec![false; input[0].len()]; input.len()];

    while y != start {
        (x, y) = (y, neighbour_of_diff_than(&input, y, x));
        loop_part[x.0][x.1] = true;
        loop_part[y.0][y.1] = true;
    }

    for ch in "|-LJ7F".chars() {
        if neighbour_by_type(ch, start).iter()
            .all(|&nei| neighbour_of(&input, nei).contains(&start)) {
            input[start.0] = input[start.0].replace('S', ch.to_string().as_str());
        }
    }

    for i in 0..input.len() {
        let mut inside = 0;
        for j in 0..input[i].len() {
            if !loop_part[i][j] {
                if inside == 1 {
                    area += 1;
                }
                continue;
            }
            inside ^= match input[i][j..].chars().nth(0).unwrap() {
                '|' => 1,
                'F' => 540,
                '7' => 540,
                'L' => 541,
                'J' => 541,
                _ => 0
            };
        }
        assert_eq!(inside, 0);
    }

    dbg!(area)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample_1() {
    let txt = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 4);
}

#[test]
fn sample_2() {
    let txt = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 4);
}

#[test]
fn sample_3() {
    let txt = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 8);
}

#[test]
fn sample_4() {
    let txt = "\
....................
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 10);
}
