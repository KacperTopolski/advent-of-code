use std::cmp::{min, max};
use std::io;
use std::io::prelude::*;

type Point = (isize, isize, isize);

fn parse(line: &str) -> (Point, Point) {
    let vec: Vec<_> = line
        .split(|c: char| !c.is_digit(10))
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    assert_eq!(vec.len(), 6);
    ((vec[0], vec[1], vec[2]), (vec[3], vec[4], vec[5]))
}

fn intersect((a, b): &(Point, Point), (c, d): &(Point, Point)) -> bool {
    if max(a.0, c.0) > min(b.0, d.0) { false }
    else if max(a.1, c.1) > min(b.1, d.1) { false }
    else if max(a.2, c.2) > min(b.2, d.2) { false }
    else { true }
}

fn simulate(input: Vec<String>) -> Vec<(Point, Point)> {
    let mut cubes: Vec<_> = input.iter().map(|s| parse(s)).collect();
    cubes.sort_by_key(|x| x.0.2);

    for i in 0..cubes.len() {
        while cubes[i].0.2 > 1 {
            let down = (
                (cubes[i].0.0, cubes[i].0.1, cubes[i].0.2 - 1),
                (cubes[i].1.0, cubes[i].1.1, cubes[i].0.2 - 1)
            );
            if cubes.iter().any(|x| intersect(x, &down)) {
                break;
            }
            cubes[i] = (
                (cubes[i].0.0, cubes[i].0.1, cubes[i].0.2 - 1),
                (cubes[i].1.0, cubes[i].1.1, cubes[i].1.2 - 1)
            );
        }
    }

    cubes
}

fn count_fall(cubes: &Vec<(Point, Point)>, st: usize) -> usize {
    let mut fallen = vec![0; cubes.len()];
    fallen[st] = 1;

    for i in st + 1..cubes.len() {
        if cubes[i].0.2 == 1 {
            continue;
        }
        let down = (
            (cubes[i].0.0, cubes[i].0.1, cubes[i].0.2 - 1),
            (cubes[i].1.0, cubes[i].1.1, cubes[i].0.2 - 1)
        );
        if (0..cubes.len()).all(|j| fallen[j] == 1 || !intersect(&down, &cubes[j])) {
            fallen[i] = 1;
        }
    }

    fallen.iter().sum()
}

fn solve(input: Vec<String>) -> usize {
    let cubes = simulate(input);

    (0..cubes.len()).map(|i| count_fall(&cubes, i) - 1).sum()
}

fn main() {
    let ans = solve(io::stdin().lock().lines().flatten().collect());
    dbg!(ans);
}

#[test]
fn sample() {
    let txt = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    dbg!(ans);
    assert_eq!(ans, 7);
}
