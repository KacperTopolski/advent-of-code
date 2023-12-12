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

fn solve_vert(input: Vec<String>, mul: i64) -> i64 {
    let mut ans = 0;
    let mut cnt = 0;
    let mut sum = 0;

    for row in input.iter() {
        let gal = row.chars().filter(|c| *c == '#').count() as i64;

        ans += gal * sum;
        cnt += gal;
        sum += cnt * if gal == 0 {mul} else {1};
    }

    dbg!(ans)
}

fn solve(input: Vec<String>, mul: i64) -> i64 {
    dbg!(solve_vert(transpose(&input), mul) + solve_vert(input, mul))
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect(), 1_000_000);
}

#[test]
fn sample() {
    let txt = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect(), 10);
    assert_eq!(ans, 1030);
}

#[test]
fn sample_2() {
    let txt = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect(), 100);
    assert_eq!(ans, 8410);
}
