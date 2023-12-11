use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;

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

fn solve(input: Vec<String>) -> usize {
    let commands = input[0].as_str();

    let mp = input
        .iter()
        .skip(2)
        .map(parse)
        .collect::<BTreeMap<_, _>>();

    let mut curr = "AAA";
    for i in 0..10_000_000 {
        curr = get_next(curr, i, &commands, &mp);
        if curr == "ZZZ" {
            return dbg!(i + 1);
        }
    }
    panic!()
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 2);
}

#[test]
fn sample2() {
    let txt = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 6);
}
