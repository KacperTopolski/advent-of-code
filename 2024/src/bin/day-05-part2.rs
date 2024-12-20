use std::io;
use std::io::prelude::*;
use itertools::Itertools;
use std::collections::HashSet;

fn parse_rule(input: &str) -> (i64, i64) {
    let split: Vec<i64> = input.split("|").map(|x| x.parse::<i64>()).flatten().collect();
    (split[0], split[1])
}

fn parse_order(input: &str) -> Vec<i64> {
    input.split(",").map(|x| x.parse::<i64>()).flatten().collect()
}

fn solve(input: Vec<String>) -> i64 {
    let split = input.iter().find_position(|x| x.is_empty()).unwrap().0;
    let (rules, orders) = input.split_at(split);

    let rules: HashSet<_> = rules.iter().map(|x| parse_rule(x)).collect();
    let orders: Vec<_> = orders.iter().skip(1).map(|x| parse_order(x)).collect();

    let score = |mut order: Vec<i64>| {
        let mut changed = false;
        let mut conti = true;
        while conti {
            conti = false;

            for j in 0..order.len() {
                for i in 0..j {
                    if !rules.contains(&(order[j], order[i])) {
                        continue;
                    }

                    order[i] ^= order[j];
                    order[j] ^= order[i];
                    order[i] ^= order[j];

                    changed = true;
                    conti = true;
                }
            }
        }
        if changed { Some(order[order.len() / 2]) } else { None }
    };

    let r = orders.into_iter()
        .filter_map(|x| score(x))
        .sum();
    dbg!(r)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 123);
}
