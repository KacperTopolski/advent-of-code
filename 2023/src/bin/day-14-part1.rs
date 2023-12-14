use std::io;
use std::io::prelude::*;

fn moveel(mut input: &mut Vec<String>, i: usize, j: usize, dx: i64, dy: i64) {
    let i = i.wrapping_sub(dx as usize);
    let j = j.wrapping_sub(dy as usize);
    if i >= input.len() || j >= input[0].len() {
        return;
    }
    if input[i][j..].chars().nth(0).unwrap() == '.' {
        unsafe {
            input[i].as_mut_vec()[j] = 'O' as u8;
            input[i.wrapping_add(dx as usize)].as_mut_vec()[j.wrapping_add(dy as usize)] = '.' as u8;
        }
        moveel(&mut input, i, j, dx, dy);
    }
}

fn solve_dir(mut input: &mut Vec<String>, dx: i64, dy: i64) {
    let mut pos: Vec<_> = (0..input.len())
        .map(|i| (0..input[0].len()).map(|j| (i, j)).collect::<Vec<_>>())
        .flatten()
        .filter(|&(i, j)| input[i].chars().nth(j).unwrap() == 'O')
        .collect();

    pos.sort_by_key(|&(i, j)| i as i64 * dx + j as i64 * dy);

    pos.iter().for_each(|&(i, j)| moveel(&mut input, i, j, dx, dy));
}

fn solve(mut input: Vec<String>) -> usize {
    solve_dir(&mut input, 1, 0);

    let ans = input
        .iter()
        .enumerate()
        .map(|(idx, st)| {
            st.chars().filter(|&c| c == 'O').count() * (input.len() - idx)
        }).sum();
    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 136);
}
