use std::cmp::max;
use std::io;
use std::io::prelude::*;

fn solve(input: Vec<String>) -> i64 {
    let ans = input
        .iter()
        .map(|line|
            line.split(' ').skip(1)
                .zip(line.split(' '))
                .map(|(line_item, line_cnt)|
                    match line_cnt.parse::<i64>() {
                        Ok(val) => match line_item.trim_end_matches([';', ',']) {
                            "red" => (val, 0, 0),
                            "green" => (0, val, 0),
                            "blue" => (0, 0, val),
                            _ => panic!()
                        },
                        Err(_) => (0, 0, 0)
                    })
                .reduce(|(a, b, c), (d, e, f)| (max(a, d), max(b, e), max(c, f)))
                .unwrap()
        )
        .map(|(a, b, c)| a * b * c)
        .sum();
    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().map(|x| x.unwrap()).collect());
}

#[test]
fn sample() {
    let txt = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 2286);
}
