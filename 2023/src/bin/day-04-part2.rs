use std::io;
use std::io::prelude::*;

fn score(line: &String) -> i64 {
    let (mine, other) = line.split_once(':').unwrap().1.split_once('|').unwrap();
    let mine: Vec<i64> = mine.split(' ').map(|x| x.parse::<i64>()).flatten().collect();
    let other: Vec<i64> = other.split(' ').map(|x| x.parse::<i64>()).flatten().collect();
    dbg!(&mine);
    dbg!(&other);

    let mut ans = 0;
    for i in mine.iter() {
        if other.contains(i) {
            ans += 1;
        }
    }

    ans
}

fn solve(input: Vec<String>) -> i64 {
    let per_card: Vec<i64> = input.iter().map(score).collect();
    let mut multiplicity = vec![1i64; input.len()];

    for i in 0..input.len() {
        for j in i+1..i+1+per_card[i] as usize {
            multiplicity[j] += multiplicity[i];
        }
    }

    dbg!(&per_card, &multiplicity);

    let ans = multiplicity.iter().sum();
    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 30);
}
