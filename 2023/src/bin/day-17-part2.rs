use std::collections::{BTreeSet, HashMap};
use std::io;
use std::io::prelude::*;
use crate::Dir::RIGHT;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

type State = (usize, usize, Dir, usize);

fn can_visit(input: &Vec<String>, dist: &mut HashMap<State, usize>, que: &mut BTreeSet<(usize, State)>, (i, j, dir, here): State, dist_st: usize) {
    if i >= input.len() || j >= input[0].len() {
        return;
    }
    let dist_st: usize = dist_st + input[i][j..].chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
    let st = (i, j, dir, here);

    if dist.get(&st).cloned().unwrap_or(1_000_000) > dist_st {
        dist.insert(st, dist_st);
        que.insert((dist_st, st));
    }
}

fn solve(input: Vec<String>) -> usize {
    let mut best = 1_000_000;

    let mut dist: HashMap<State, usize> = HashMap::new();
    let mut que: BTreeSet<(usize, State)> = BTreeSet::new();

    let start: State = (0, 0, RIGHT, 0);
    dist.insert(start, 0);
    que.insert((0, start));
    let start: State = (0, 0, Dir::DOWN, 0);
    dist.insert(start, 0);
    que.insert((0, start));


    while let Some((dist_st, st)) = que.pop_first() {
        let (i, j, dir, here) = st;
        if dist.get(&st).cloned().unwrap() != dist_st {
            continue;
        }

        if i+1 == input.len() && j+1 == input[0].len() && here >= 4 {
            best = best.min(dist_st);
        }

        let GO_LEFT_SAME = (i, j.wrapping_sub(1), Dir::LEFT, here + 1);
        let GO_RIGHT_SAME = (i, j + 1, Dir::RIGHT, here + 1);
        let GO_UP_SAME = (i.wrapping_sub(1), j, Dir::UP, here + 1);
        let GO_DOWN_SAME = (i + 1, j, Dir::DOWN, here + 1);

        let GO_LEFT = (i, j.wrapping_sub(1), Dir::LEFT, 1);
        let GO_RIGHT = (i, j + 1, Dir::RIGHT, 1);
        let GO_UP = (i.wrapping_sub(1), j, Dir::UP, 1);
        let GO_DOWN = (i + 1, j, Dir::DOWN, 1);

        if here <= 9 {
            match dir {
                Dir::UP => can_visit(&input, &mut dist, &mut que, GO_UP_SAME, dist_st),
                Dir::DOWN => can_visit(&input, &mut dist, &mut que, GO_DOWN_SAME, dist_st),
                Dir::LEFT => can_visit(&input, &mut dist, &mut que, GO_LEFT_SAME, dist_st),
                Dir::RIGHT => can_visit(&input, &mut dist, &mut que, GO_RIGHT_SAME, dist_st),
            }
        }
        if here >= 4 {
            match dir {
                Dir::UP | Dir::DOWN => { can_visit(&input, &mut dist, &mut que, GO_LEFT, dist_st); can_visit(&input, &mut dist, &mut que, GO_RIGHT, dist_st); }
                Dir::LEFT | Dir::RIGHT => { can_visit(&input, &mut dist, &mut que, GO_UP, dist_st); can_visit(&input, &mut dist, &mut que, GO_DOWN, dist_st); }
            }
        }
    }

    dbg!(best)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 94);
}

#[test]
fn sample_2() {
    let txt = "\
111111111111
999999999991
999999999991
999999999991
999999999991";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 71);
}
