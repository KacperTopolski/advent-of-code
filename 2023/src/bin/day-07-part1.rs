use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn card_weight(ch: char) -> i8 {
    match ch {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        d => d.to_digit(10).unwrap() as i8
    }
}

fn hand_type(data: &str) -> i8 {
    let mut mp = BTreeMap::<char, i8>::new();

    for ch in data.chars() {
        mp.insert(ch, *mp.get(&ch).unwrap_or(&0) + 1);
    }

    let mut srt: Vec<&i8> = mp.values().collect();
    srt.sort();
    srt.reverse();

    match srt[..] {
        [5] => 6,
        [4, 1] => 5,
        [3, 2] => 4,
        [3, ..] => 3,
        [2, 2, 1] => 2,
        [2, 1, 1, 1] => 1,
        [1, 1, 1, 1, 1] => 0,
        [..] => panic!()
    }
}

fn to_key(data: &str) -> [i8; 6] {
    let mut it = data.chars();
    [
        hand_type(data),
        card_weight(it.next().unwrap()),
        card_weight(it.next().unwrap()),
        card_weight(it.next().unwrap()),
        card_weight(it.next().unwrap()),
        card_weight(it.next().unwrap())
    ]
}

fn solve(input: Vec<String>) -> i64 {
    let mut input: Vec<_> = input
        .iter()
        .map(|s| s.split_once(' '))
        .flatten()
        .map(|(a, b)| (to_key(&a), b.parse::<i64>().unwrap()))
        .collect();
    input.sort();
    dbg!(input
        .iter()
        .enumerate()
        .map(|(idx, &(_, bet))| (idx as i64 + 1) * bet)
        .sum()
    )
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 6440);
}
