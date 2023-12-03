use std::io;
use std::io::prelude::*;

fn solve(input: Vec<String>) -> i64 {
    let mut nums: Vec<Vec<Option<(i64, usize, usize, usize)>>> = vec![vec![None; input[0].len()]; input.len()];

    for row in 0..input.len() {
        for r in 0..=input[0].len() {
            for l in (0..r).rev() {
                let num = input[row][l..r].parse::<i64>();
                if num.is_err() {
                    continue;
                }
                let num = num.unwrap();
                for i in l..r {
                    nums[row][i] = Some((num, l, r, row));
                }
            }
        }
    }

    let mut ans = 0;
    for row in 0..input.len() {
        for (col, ch) in input[row].chars().enumerate() {
            if ch != '*' {
                continue
            }
            let mut nums_around: Vec<(i64, usize, usize, usize)> = vec![];
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    let i = row as i64 + dx;
                    let j = col as i64 + dy;
                    if 0 <= i && i < input.len() as i64 && 0 <= j && j < input[0].len() as i64 {
                        match nums[i as usize][j as usize] {
                            Some(val) => nums_around.push(val),
                            _ => {}
                        }
                    }
                }
            }
            dbg!(row, col, &nums_around);
            nums_around.dedup();
            if nums_around.len() == 2 {
                ans += nums_around[0].0 * nums_around[1].0;
            }
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
    assert_eq!(ans, 467835);
}
