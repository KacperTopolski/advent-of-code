use std::io;
use std::io::prelude::*;

fn transpose(input: &Vec<String>) -> Vec<String> {
    let mut out = vec!["".to_string(); input[0].len()];

    for row in input.iter() {
        for (idx, ch) in row.chars().enumerate() {
            out[idx] += ch.to_string().as_str();
        }
    }

    out
}

fn has_reflection(input: &Vec<String>, reflection: usize) -> bool {
    (0..input.len()).all(|idx| {
        let oidx = (2 * reflection - 1).wrapping_sub(idx);
        input[idx] == input[if oidx < input.len() {oidx} else {idx}]
    })
}

fn solve_rows(input: Vec<String>, mul: i64) -> i64 {
    dbg!(&input);
    dbg!((1..input.len()).filter(|&r| has_reflection(&input, r)).sum::<usize>()) as i64 * mul
}

fn solve_map(input: Vec<String>) -> i64 {
    dbg!(solve_rows(transpose(&input), 1) + solve_rows(input, 100))
}

fn solve(input: Vec<String>) -> i64 {
    let ans = input
        .split(|x| x.is_empty())
        .map(|x| solve_map(x.to_vec()))
        .sum();

    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 405);
}

#[test]
fn mine_1() {
    let txt = "\
##
##";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 101);
}

#[test]
fn mine_2() {
    let txt = "\
###
###
###";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 303);
}

#[test]
fn mine_3() {
    let txt = "\
#.#
...
#.#";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 0);
}

#[test]
fn mine_4() {
    let txt = "\
#..#
....
....
#..#";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 202);
}
