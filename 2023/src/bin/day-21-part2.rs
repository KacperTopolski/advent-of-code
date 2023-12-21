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

fn solve_from(input: &Vec<String>, max_dist: isize, si: isize, sj: isize) -> Vec<usize> {
    let mut dq: VecDeque<(isize, isize, usize)> = VecDeque::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();

    let mut ans = vec![0usize; max_dist as usize];
    dq.push_back((si, sj, 0));

    while let Some((i, j, dist)) = dq.pop_front() {
        if dist >= ans.len() {
            break;
        }
        let ij = (i, j);
        if seen.contains(&ij) || !access(&input, i, j) {
            continue;
        }
        ans[dist] += 1;
        seen.insert((i, j));
        dq.push_back((i - 1, j, dist + 1));
        dq.push_back((i + 1, j, dist + 1));
        dq.push_back((i, j - 1, dist + 1));
        dq.push_back((i, j + 1, dist + 1));
    }
    for i in 2..ans.len() {
        ans[i] += ans[i-2];
    }
    ans
}

fn rot(input: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = input.clone();
    let n = out.len();

    for i in 0..n {
        for j in 0..n {
            unsafe {
                out[j].as_mut_vec()[n-1-i] = input[i].as_bytes()[j];
            }
        }
    }

    out
}

const M: isize = 1000;

fn solve_right(input: &Vec<String>, mut more: isize) -> u128 {
    let cached = solve_from(input, M + 1, (input.len() / 2) as isize, 0);

    let get = |val: isize| {
        if val < cached.len() as isize {
            cached[val as usize]
        }
        else if val % 2 == M % 2 {
            cached[M as usize]
        }
        else {
            cached[M as usize - 1]
        }
    };
    more -= input.len() as isize / 2 + 1;

    let mut ans: usize = 0;
    while more >= 0 {
        ans += get(more);

        more -= input.len() as isize;
    }

    ans as u128
}

fn solve_right_down(input: &Vec<String>, mut more: isize) -> u128 {
    let cached = solve_from(input, M + 1, 0, 0);
    more -= (input.len() as isize / 2 + 1) * 2;
    let mut ans = 0;

    let get = |val: isize| {
        if val < 0 {
            0
        }
        else if val < cached.len() as isize {
            cached[val as usize] as u128
        }
        else if val % 2 == M % 2 {
            cached[M as usize] as u128
        }
        else {
            cached[M as usize - 1] as u128
        }
    };

    let upbound = more / (input.len() as isize) + 10;

    for i in 0..upbound {
        ans += (i + 1) as u128 * get(more - i * (input.len() as isize));
    }

    ans
}

fn solve(mut input: Vec<String>, more: isize) -> u128 {
    let n = input.len();
    assert_eq!(n, input[0].len());
    assert_eq!(1, n % 2);
    assert_eq!(find_start(&input), ((n / 2) as isize, (n / 2) as isize));

    let mut ans = solve_from(&input, M + 1, (n / 2) as isize, (n / 2) as isize)[(M - more % 2) as usize] as u128;
    dbg!(ans);

    for _ in 0..4 {
        ans += solve_right(&input, more);
        ans += solve_right_down(&input, more);

        input = rot(input);
    }

    ans
}

fn main() {
    let ans = solve(io::stdin().lock().lines().flatten().collect(), 26501365);
    dbg!(ans);
}

#[test]
fn mine_1() {
    let txt = "\
...
.S.
...";
    let m = 3;
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect(), m);
    dbg!(ans);
    assert_eq!(ans, ((m + 1) * (m + 1)) as u128);
}

#[test]
fn mine_2() {
    let txt = "\
...
.S.
...";
    let m = 4;
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect(), m);
    dbg!(ans);
    assert_eq!(ans, ((m + 1) * (m + 1)) as u128);
}

#[test]
fn mine_3() {
    let txt = "\
...
.S.
...";
    let m = 5;
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect(), m);
    dbg!(ans);
    assert_eq!(ans, ((m + 1) * (m + 1)) as u128);
}
