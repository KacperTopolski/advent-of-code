use std::io;
use std::io::prelude::*;

fn hash(to_hash: &str) -> usize {
    let mut h = 0;
    for ch in to_hash.chars() {
        h = (h + ch as usize) * 17 % 256;
    }
    h
}

fn solve(input: Vec<String>) -> usize {
    let mut hashmap: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

    for line in input[0].split(',') {
        if line.ends_with('-') {
            let to_hash = &line[..line.len()-1];
            hashmap[hash(to_hash)].retain(|tup| tup.0 != to_hash);
            continue;
        }
        let (to_hash, num) = line.split_once('=').unwrap();
        let num = num.parse().unwrap();
        let bucket = &mut hashmap[hash(to_hash)];

        match bucket.iter_mut().find(|tup| tup.0 == to_hash) {
            None => bucket.push((to_hash.to_string(), num)),
            Some(it) => it.1 = num
        }
    }

    let ans = hashmap
        .iter()
        .map(|bucket| {

            bucket
                .iter()
                .enumerate()
                .map(|(idx, val)| (idx + 1) * val.1)
                .sum::<usize>()

        })
        .enumerate()
        .map(|(idx, val)| (idx + 1) * val)
        .sum::<usize>();

    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 145);
}
