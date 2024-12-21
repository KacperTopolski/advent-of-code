use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::iter::once;
use itertools::Itertools;

fn dbg_print(vec: &Vec<i64>) -> String {
    vec.iter().map(|i| if *i < 0 { ".".to_string() } else { i.to_string() }).join("")
}

fn solve(input: Vec<String>) -> i64 {
    let input: &str = &input[0];

    let mut space: Vec<i64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(id, digit)| vec![if id % 2 == 0 { (id / 2) as i64 } else { -1 }; digit as usize])
        .collect();

    dbg!(dbg_print(&space));

    let mut m_pos: i64 = space.len() as i64 - 1;
    let mut hs: HashSet<i64> = once(-1).collect();
    while m_pos >= 0 {
        while hs.contains(&space[m_pos as usize]) {
            m_pos -= 1;
        }

        let mov_val = space[m_pos as usize];
        hs.insert(mov_val);
        let mut mov_cnt = 0;
        while space.get(m_pos as usize) == Some(&mov_val) {
            m_pos -= 1;
            mov_cnt += 1;
        }

        let mut free_space = 0;
        let mut i = 0;

        while free_space < mov_cnt && i != space.len() {
            if space[i] != -1 {
                free_space = 0;
            }
            else {
                free_space += 1;
            }
            i += 1;
        }
        if free_space != mov_cnt || i as i64 > m_pos + 1 {
            continue;
        }

        for k in 0..mov_cnt {
            space[i-1-k] = mov_val;
            space[m_pos as usize+1+k] = -1;
        }
    }

    let r = space.iter().enumerate().map(|(x, y)| x as i64 * (*y).max(0)).sum();
    dbg!(r)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "2333133121414131402";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 2858);
}
