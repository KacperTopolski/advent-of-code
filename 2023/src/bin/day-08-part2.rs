use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn gcd(a: u64, b: u64) -> u64 {
    if a > b {
        return gcd(b, a);
    }
    match a {
        0 => b,
        _ => gcd(b % a, a)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn parse(input: &String) -> (&str, (&str, &str)) {
    (&input[0..3], (&input[7..10], &input[12..15]))
}

fn get_next<'a>(curr: &str, idx: usize, commands: &str, mp: &'a BTreeMap<&str, (& str, &str)>) -> &'a str {
    let go_right = commands[idx % commands.len()..].chars().nth(0).unwrap() == 'R';
    match mp.get(&curr) {
        None => panic!(),
        Some(&(l, r)) => match go_right {
            true => r,
            false => l
        }
    }
}

fn to_first(mut start: String, commands: &str, mp: &BTreeMap<&str, (& str, &str)>) -> u64 {
    for i in 0..10_000_000 {
        start = get_next(start.as_str(), i, &commands, &mp).to_string();
        if start.chars().nth(2).unwrap() == 'Z' {
            return dbg!(i + 1) as u64;
        }
    }
    panic!()
}

fn solve(input: Vec<String>) -> u64 {
    let commands = input[0].as_str();

    let mp = input
        .iter()
        .skip(2)
        .map(parse)
        .collect::<BTreeMap<_, _>>();

    let ans = mp
        .keys()
        .map(|&s| s)
        .filter(|s| s.chars().nth(2).unwrap() == 'A')
        .map(|s| to_first(s.to_string(), &commands, &mp))
        .reduce(lcm)
        .unwrap();

    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 6);
}
