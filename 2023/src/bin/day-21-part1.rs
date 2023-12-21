use std::collections::{HashSet, VecDeque};
use std::io;
use std::io::prelude::*;

fn find_start(input: &Vec<String>) -> (isize, isize) {
    for (i, row) in input.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            if ch == 'S' {
                return (i as isize, j as isize);
            }
        }
    }
    panic!()
}

fn access(input: &Vec<String>, i: isize, j: isize) -> bool {
    if i < 0 || j < 0 || i >= input.len() as isize || j >= input[0].len() as isize {
        return false;
    }
    return input[i as usize][j as usize..].chars().next().unwrap() != '#';
}

fn solve(input: Vec<String>, more: usize) -> usize {
    let mut dq: VecDeque<(isize, isize, usize)> = VecDeque::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut ans = 0;

    let (si, sj) = find_start(&input);
    dq.push_back((si, sj, 0));

    while let Some((i, j, dist)) = dq.pop_front() {
        if dist > more {
            break;
        }
        let ij = (i, j);
        if seen.contains(&ij) || !access(&input, i, j) {
            continue;
        }
        if dist % 2 == more % 2 {
            ans += 1;
        }
        seen.insert((i, j));
        dq.push_back((i - 1, j, dist + 1));
        dq.push_back((i + 1, j, dist + 1));
        dq.push_back((i, j - 1, dist + 1));
        dq.push_back((i, j + 1, dist + 1));
    }

    ans
}

fn main() {
    let ans = solve(io::stdin().lock().lines().flatten().collect(), 64);
    dbg!(ans);
}

#[test]
fn sample() {
    let txt = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect(), 6);
    dbg!(ans);
    assert_eq!(ans, 16);
}
