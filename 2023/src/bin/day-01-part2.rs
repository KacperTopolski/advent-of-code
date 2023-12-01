use std::io;
use std::io::prelude::*;

fn find_first_digit(s: String, words: &Vec<String>) -> usize {
    let mut s = s.as_str();
    while !s.is_empty() {
        let first: char = s.chars().nth(0).unwrap();
        if first.is_digit(10) {
            return first.to_digit(10).unwrap() as usize;
        }
        for (idx, val) in words.iter().enumerate() {
            if s.starts_with(val) {
                return idx;
            }
        }
        s = &s[1..];
    }
    panic!();
}

fn solve<I: std::iter::Iterator<Item = String>>(iter: I) -> usize {
    let words: Vec<String> = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
        .iter()
        .map(|x| x.to_string())
        .collect();

    let rev_words: Vec<String> = words
        .iter()
        .map(|x| x.chars().rev().collect())
        .collect();

    let mut ans = 0;
    for line in iter {
        let last = find_first_digit(line.chars().rev().collect(), &rev_words);
        let first = find_first_digit(line, &words);
        ans += first * 10 + last;
    }
    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().map(|x| x.unwrap()));
}

#[test]
fn it_works() {
    let words = ["two1nine", "eightwothree", "abcone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen"];
    let ans = solve(words.iter().map(|x| x.to_string()));
    assert_eq!(ans, 281);
}

#[test]
fn test_pasting() {
    let txt = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let ans = solve(txt.split('\n').map(|x| x.to_string()));
    assert_eq!(ans, 281);
}
