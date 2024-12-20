use std::io;
use std::io::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;

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

    let rules: Vec<_> = rules.iter().map(|x| parse_rule(x)).collect();
    let orders: Vec<_> = orders.iter().skip(1).map(|x| parse_order(x)).collect();

    let check = |order: &Vec<i64>| {
        let lookup: HashMap<_, _> = order.iter().enumerate().map(|(x, y)| (*y, x)).collect();
        for rule in &rules {
            if let (Some(i), Some(j)) = (lookup.get(&rule.0), lookup.get(&rule.1)) {
                if i > j {
                    return false;
                }
            }
        }
        true
    };

    let r = orders.iter()
        .filter(|x| check(x))
        .map(|v| v[v.len() / 2])
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
    assert_eq!(ans, 143);
}
