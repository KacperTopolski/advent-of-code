use std::io;
use std::io::prelude::*;

fn symbols_around(input: &Vec<Vec<char>>, x: i64, y: i64) -> bool {
    for i in x-1..=x+1 {
        for j in y-1..=y+1 {
            if 0 <= i && i < input.len() as i64 && 0 <= j && j < input[0].len() as i64 {
                let ch: char = input[i as usize][j as usize];
                if !ch.is_digit(10) && ch != '.' {
                    return true;
                }
            }
        }
    }

    false
}
fn score(input: &Vec<Vec<char>>, mut i: usize, mut j: usize, dx: usize, dy: usize) -> i64 {
    let mut ans: i64 = 0;
    let mut has_sym = false;

    if i >= dx && j >= dy && input[i - dx][j - dy].is_digit(10) {
        return 0;
    }
    while i < input.len() && j < input[0].len() && input[i][j].is_digit(10) {
        ans = 10 * ans + input[i][j].to_digit(10).unwrap() as i64;
        has_sym |= symbols_around(&input, i as i64, j as i64);
        i += dx;
        j += dy;
    }

    match has_sym {
        true => ans,
        false => 0
    }
}


fn solve(input: Vec<String>) -> i64 {
    let input: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    let mut ans = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            ans += score(&input, i, j, 0, 1);
        }
    }
    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().map(|x| x.unwrap()).collect());
}

#[test]
fn sample() {
    let txt = "\
467..1140.
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 4361);
}
