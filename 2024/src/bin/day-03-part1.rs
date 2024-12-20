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
    let input = input.join("");

    let r = (0..input.len()).map(|i| input.split_at(i).1).filter_map(|s| try_to_parse(s)).sum();
    dbg!(r)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 161);
}
