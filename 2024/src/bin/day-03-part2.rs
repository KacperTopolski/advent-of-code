use std::io;
use std::io::prelude::*;

fn try_to_parse_i(input: &str) -> Option<i64> {
    input.parse().ok()
}

fn try_to_parse(input: &str) -> Option<i64> {
    if let Some(input) = input.strip_prefix("mul(") {
        if let Some(i) = input.find(")") {
            let input = input.split_at(i).0;
            if let Some((x, y)) = input.split_once(",") {
                if let (Some(x), Some(y)) = (try_to_parse_i(x), try_to_parse_i(y)) {
                    return Some(x * y)
                }
            }
        }
    }
    None
}

fn solve(input: Vec<String>) -> i64 {
    let mut input: &str = &input.join("");
    let mut r = 0;
    let mut active = true;

    while !input.is_empty() {
        if input.starts_with("do()") {
            active = true;
        }
        if input.starts_with("don't()") {
            active = false;
        }
        if active {
            r += try_to_parse(input).unwrap_or_default()
        }

        input = input.split_at(1).1
    }

    dbg!(r)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 48);
}
