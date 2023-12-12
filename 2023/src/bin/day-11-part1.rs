use std::io;
use std::io::prelude::*;

fn process_row(input: &String) -> Vec<String> {
    let cnt = if input.chars().all(|c| c == '.') {2} else {1};
    return vec![input.clone(); cnt];
}

fn double_row(input: Vec<String>) -> Vec<String> {
    input
        .iter()
        .map(process_row)
        .flatten()
        .collect()
}

fn transpose(input: Vec<String>) -> Vec<String> {
    let mut out = vec!["".to_string(); input[0].len()];

    for row in input.iter() {
        for (idx, ch) in row.chars().enumerate() {
            out[idx] += ch.to_string().as_str();
        }
    }

    out
}

fn solve(input: Vec<String>) -> i64 {
    let input = double_row(transpose(double_row(transpose(input))));
    let mut ans = 0;

    for (i, row_i) in input.iter().enumerate() {
        for (j, ch_ij) in row_i.chars().enumerate() {
            for (k, row_k) in input.iter().enumerate() {
                for (l, ch_kl) in row_k.chars().enumerate() {
                    if ch_ij == '#' && ch_kl == '#' {
                        ans += (i.abs_diff(k) + j.abs_diff(l)) as i64;
                    }
                }
            }
        }
    }

    dbg!(ans / 2)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
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
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 374);
}
