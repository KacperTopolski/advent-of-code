use std::collections::{BTreeMap, BTreeSet, HashMap};
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

fn parse_color(line: &str) -> (i64, i64, i64) {
    let color = &line.rsplit_once("#").unwrap().1[..6];
    let (dx, dy) = match color.chars().last().unwrap() {
        '0' => (0, 1),
        '1' => (1, 0),
        '2' => (0, -1),
        '3' => (-1, 0),
        _ => panic!()
    };
    let mut val = 0;
    for ch in color.chars() {
        val = 16 * val + ch.to_digit(16).unwrap() as i64;
    }
    (dx, dy, val / 16)
}

fn solve(input: Vec<String>) -> i64 {
    let input: Vec<(i64, i64, i64)> = input.iter().map(|s| parse_color(&s)).collect();
    // dbg!(&input);

    let mut cuts: BTreeSet<i64> = BTreeSet::new();
    let (mut px, mut py): (i64, i64) = (0, 0);

    for (dx, dy, cnt) in input.iter() {
        px += dx * cnt;
        py += dy * cnt;
        cuts.insert(px);
        cuts.insert(px + 1);
        cuts.insert(py);
        cuts.insert(py + 1);
    }
    assert_eq!((0, 0), (px, py));

    dbg!(cuts.len());
    let cuts: Vec<i64> = cuts.iter().cloned().collect();
    let rcuts: BTreeMap<i64, usize> = cuts.iter().enumerate().map(|v| (v.1.clone(), v.0)).collect();

    let mut dsu = DSU::new();
    let border: (i64, i64) = (rcuts[&0] as i64, rcuts[&0] as i64);
    let (mut px, mut py): (i64, i64) = (0, 0);
    let (mut pux, mut puy): (usize, usize) = (rcuts[&0], rcuts[&0]);
    for (dx, dy, cnt) in input.iter() {
        px += dx * cnt;
        py += dy * cnt;
        while rcuts[&px] != pux || rcuts[&py] != puy {
            pux = pux.wrapping_add_signed(*dx as isize);
            puy = puy.wrapping_add_signed(*dy as isize);
            dsu.merge(border, (pux as i64, puy as i64));
        }
    }

    let mx = dsu.mx();
    for i in -1..=mx + 1 {
        for j in -1..=mx + 1 {
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
    for i in 0..=mx {
        for j in 0..=mx {
            if dsu.find((i, j)) != dsu.find(bad) {
                ans += (cuts[(i+1) as usize]-cuts[i as usize])*(cuts[(j+1) as usize]-cuts[j as usize]) as i64;
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
    assert_eq!(ans, 952408144115);
}
