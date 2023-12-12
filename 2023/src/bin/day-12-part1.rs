use std::io;
use std::io::prelude::*;

fn solve_row(input: &String) -> i64 {
    let (record, info) = input.split_once(' ').unwrap();
    let record: Vec<char> = record.chars().collect();
    let info: Vec<i64> = info.split(',').map(|x| x.parse::<_>()).flatten().collect();

    let mut dp = vec![vec![0i64; info.len() + 1]; record.len()];
    let mut longest_ok = 0;

    for (idx, ch) in record.iter().enumerate() {
        longest_ok = if *ch == '.' {0} else {longest_ok + 1};

        for idx_dp in 0..=info.len() {
            if *ch != '#' {
                dp[idx][idx_dp] += if idx > 0 {dp[idx - 1][idx_dp]} else if idx_dp == 0 {1} else {0};
            }
            if idx_dp == 0 {
                continue;
            }
            let len_here = info[idx_dp - 1];
            if longest_ok < len_here {
                continue;
            }
            if idx as i64 - len_here == -1 || record[idx - len_here as usize] != '#' {
                dp[idx][idx_dp] += if idx as i64 - len_here - 1 >= 0 {dp[idx - len_here as usize - 1][idx_dp - 1]} else if idx_dp == 1 {1} else {0};
            }
        }
    }

    // dbg!(&record, &info, &dp);
    dbg!(*dp.last().unwrap().last().unwrap())
}

fn solve(input: Vec<String>) -> i64 {
    dbg!(
        input
            .iter()
            .map(solve_row)
            .sum()
    )
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample_1() {
    let txt = "\
???.### 1,1,3";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 1);
}

#[test]
fn sample() {
    let txt = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 21);
}
