use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

struct DSU {
    p: HashMap<(i64, i64), (i64, i64)>
}
impl DSU {
    fn new() -> Self {
        Self {p: HashMap::new()}
    }
    fn find(&mut self, key: (i64, i64)) -> (i64, i64) {
        let par = self.p.get(&key).cloned();
        if par.is_none() || par == Some(key) {
            return key;
        }
        let par = self.find(par.unwrap());
        self.p.insert(key, par);
        par
    }
    fn merge(&mut self, a: (i64, i64), b: (i64, i64)) {
        let a = self.find(a);
        let b = self.find(b);
        if a != b {
            self.p.insert(b, a);
        }
    }
    fn mx(&self) -> i64 {
        self.p.keys().map(|&(a, b)| a.abs().max(b.abs())).max().unwrap()
    }
}

fn solve(input: Vec<String>) -> usize {
    let input: Vec<(i64, i64, i64)> = input.iter().map(|s| {
        let (dir, s) = s.split_once(' ').unwrap();
        let dir: (i64, i64) = match dir {
            "R" => (0, 1),
            "L" => (0, -1),
            "U" => (-1, 0),
            "D" => (1, 0),
            _ => panic!()
        };
        let cnt: i64 = s.split_once(' ').unwrap().0.parse().unwrap();
        (dir.0, dir.1, cnt)
    }).collect();

    let mut dsu = DSU::new();
    let border: (i64, i64) = (0, 0);
    let (mut px, mut py): (i64, i64) = (0, 0);

    for (dx, dy, cnt) in input {
        for _ in 0..cnt {
            px += dx;
            py += dy;
            dsu.merge(border, (px, py));
        }
    }
    assert_eq!(border, (px, py));

    let mx = dsu.mx();
    for i in -mx - 1..=mx + 1 {
        for j in -mx - 1..=mx + 1 {
            for (k, l) in [(i, j + 1), (i + 1, j)] {
                if dsu.find((i, j)) != border && dsu.find((k, l)) != border {
                    dsu.merge((i, j), (k, l));
                }
            }
        }
    }
    dbg!(mx);

    let mut ans = 0;
    let bad = (mx + 1, mx + 1);
    for i in -mx - 1..=mx + 1 {
        for j in -mx - 1..=mx + 1 {
            if dsu.find((i, j)) != dsu.find(bad) {
                ans += 1;
            }
        }
    }

    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 62);
}
