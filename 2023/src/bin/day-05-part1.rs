use std::io;
use std::io::prelude::*;

fn apply_op(inp: i64, op: &Vec<(i64, i64, i64)>) -> i64 {
    for &(to, from, len) in op {
        if from <= inp && inp < from + len {
            return inp - from + to;
        }
    }

    inp
}

fn apply_ops(mut inp: i64, ops: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    for op in ops {
        inp = apply_op(inp, op);
    }

    inp
}

fn solve(input: Vec<String>) -> i64 {
    let seeds: Vec<i64> = input[0].trim_start_matches("seeds: ").split(' ').map(|x| x.parse::<i64>()).flatten().collect();
    dbg!(&seeds);

    let mut maps: Vec<Vec<(i64, i64, i64)>> = vec![];

    for line in input.iter().skip(1) {
        if line.is_empty() {
            continue;
        }
        if !line.chars().nth(0).unwrap().is_digit(10) {
            maps.push(vec![]);
            continue;
        }
        let line: Vec<i64> = line.split(' ').map(|x| x.parse::<i64>()).flatten().collect();
        assert_eq!(line.len(), 3);
        maps.last_mut().unwrap().push((line[0], line[1], line[2]));
    }
    dbg!(&maps);

    dbg!(seeds.iter().map(|seed| apply_ops(*seed, &maps)).min().unwrap())
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 35);
}
