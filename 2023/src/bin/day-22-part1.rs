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

fn solve(input: Vec<String>) -> usize {
    let mut cubes: Vec<_> = input.iter().map(|s| parse(s)).collect();
    let mut support = vec![0; cubes.len()];
    cubes.sort_by_key(|x| x.0.2);
    // dbg!(&cubes);

    for i in 0..cubes.len() {
        while cubes[i].0.2 > 1 {
            let down = (
                (cubes[i].0.0, cubes[i].0.1, cubes[i].0.2 - 1),
                (cubes[i].1.0, cubes[i].1.1, cubes[i].0.2 - 1)
            );
            let support_here: Vec<_> = cubes.iter()
                .enumerate()
                .filter(|x| intersect(x.1, &down))
                .map(|x| x.0)
                .collect();

            if support_here.len() == 1 {
                support[support_here[0]] += 1;
            }
            if !support_here.is_empty() {
                // dbg!(i, support_here);
                break;
            }

            cubes[i] = (
                (cubes[i].0.0, cubes[i].0.1, cubes[i].0.2 - 1),
                (cubes[i].1.0, cubes[i].1.1, cubes[i].1.2 - 1)
            );
        }
    }

    dbg!(&cubes);
    dbg!(&support);
    support.iter().filter(|x| **x == 0).count()
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
    assert_eq!(ans, 5);
}

#[test]
fn mine() {
    let txt = "\
0,0,1~0,0,1
0,0,2~0,0,2";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    dbg!(ans);
    assert_eq!(ans, 1);
}
