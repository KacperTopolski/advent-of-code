use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

type State = (usize, usize, Dir);

fn next_raw(mp: &Vec<String>, s: State) -> Vec<State> {
    let ch = mp[s.0].as_bytes()[s.1] as char;
    // dbg!(ch);

    match s.2 {
        Dir::UP => match ch {
            '-' => vec![(s.0, s.1.wrapping_sub(1), Dir::LEFT), (s.0, s.1 + 1, Dir::RIGHT)],
            '\\' => vec![(s.0, s.1.wrapping_sub(1), Dir::LEFT)],
            '/' => vec![(s.0, s.1 + 1, Dir::RIGHT)],
            _ => vec![(s.0.wrapping_sub(1), s.1, Dir::UP)]
        }
        Dir::DOWN => match ch {
            '-' => vec![(s.0, s.1.wrapping_sub(1), Dir::LEFT), (s.0, s.1 + 1, Dir::RIGHT)],
            '\\' => vec![(s.0, s.1 + 1, Dir::RIGHT)],
            '/' => vec![(s.0, s.1.wrapping_sub(1), Dir::LEFT)],
            _ => vec![(s.0 + 1, s.1, Dir::DOWN)]
        }
        Dir::LEFT => match ch {
            '|' => vec![(s.0.wrapping_sub(1), s.1, Dir::UP), (s.0 + 1, s.1, Dir::DOWN)],
            '\\' => vec![(s.0.wrapping_sub(1), s.1, Dir::UP)],
            '/' => vec![(s.0 + 1, s.1, Dir::DOWN)],
            _ => vec![(s.0, s.1.wrapping_sub(1), Dir::LEFT)]
        }
        Dir::RIGHT => match ch {
            '|' => vec![(s.0.wrapping_sub(1), s.1, Dir::UP), (s.0 + 1, s.1, Dir::DOWN)],
            '\\' => vec![(s.0 + 1, s.1, Dir::DOWN)],
            '/' => vec![(s.0.wrapping_sub(1), s.1, Dir::UP)],
            _ => vec![(s.0, s.1 + 1, Dir::RIGHT)]
        }
    }
}


fn next(mp: &Vec<String>, s: State) -> Vec<State> {
    next_raw(mp, s)
        .iter()
        .cloned()
        .filter(|(i, j, _)| i < &mp.len() && j < &mp[0].len())
        .collect()
}

fn solve_start(input: &Vec<String>, start: State) -> usize {
    let mut to_see: Vec<State> = vec![start];
    let mut seen_states: HashSet<State> = HashSet::from([start]);
    let mut seen_tiles: HashSet<(usize, usize)> = HashSet::from([(start.0, start.1)]);

    while let Some(s) = to_see.pop() {
        // dbg!(s);
        for next_state in next(&input, s) {
            if seen_states.contains(&next_state) {
                continue;
            }
            to_see.push(next_state);
            seen_states.insert(next_state);
            seen_tiles.insert((next_state.0, next_state.1));
        }
    }

    seen_tiles.len()
}

fn solve(input: Vec<String>) -> usize {
    let itr = (0..input.len()).map(|i| (i, 0, Dir::RIGHT));
    let itl = (0..input.len()).map(|i| (i, input[0].len() - 1, Dir::LEFT));
    let itd = (0..input[0].len()).map(|j| (0, j, Dir::DOWN));
    let itu = (0..input[0].len()).map(|j| (input.len() - 1, j, Dir::UP));

    let ans = itl
        .chain(itr)
        .chain(itd)
        .chain(itu)
        .map(|s| solve_start(&input, s))
        .max()
        .unwrap();

    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 51);
}
