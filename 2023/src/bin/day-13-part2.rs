use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

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

fn solve_rows(input: Vec<String>) -> HashSet<usize> {
    (1..input.len()).filter(|&r| has_reflection(&input, r)).collect()
}

fn solve_map(input: Vec<String>) -> (HashSet<usize>, HashSet<usize>) {
    (solve_rows(transpose(&input)), solve_rows(input))
}

fn solve_smudge(input: Vec<String>) -> usize {
    let (columns_original, rows_original) = solve_map(input.clone());

    dbg!(&columns_original, &rows_original);

    let ans: usize = (0..input.len())
        .map(|i| (0..input[0].len()).map(|j| (i, j)).collect::<Vec<_>>())
        .flatten()
        .map(|(i, j)| {
        let mut here = input.clone();
        unsafe {
            here[i].as_mut_vec()[j] ^= ('#' as u8) ^ ('.' as u8);
        }
        let (columns_here, rows_here) = solve_map(here);

        let columns_count = (&columns_here - &columns_original).iter().count();
        let rows_count = (&rows_here - &rows_original).iter().count();

        let columns_sum = (&columns_here - &columns_original).iter().sum::<usize>();
        let rows_sum = (&rows_here - &rows_original).iter().sum::<usize>();

        if columns_count + rows_count > 0 {
            dbg!(i, j, &columns_here, &rows_here);
        }

        rows_sum * 100 + columns_sum
    }).sum();

    dbg!(ans / 2)
}

fn solve(input: Vec<String>) -> usize {
    let ans = input
        .split(|x| x.is_empty())
        .map(|x| solve_smudge(x.to_vec()))
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
    assert_eq!(ans, 400);
}

#[test]
fn sample_1() {
    let txt = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 300);
}

#[test]
fn sample_2() {
    let txt = "\
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 100);
}
