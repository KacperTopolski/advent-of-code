use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn get_id(mp: &mut HashMap<String, usize>, v: &str) -> usize {
    if !mp.contains_key(v) {
        mp.insert(v.to_string(), mp.len());
    }
    mp[v]
}

const INF: usize = 9999;

fn dfs(g: &Vec<Vec<usize>>, subsz: &mut Vec<usize>, over_cnt: &mut Vec<usize>, par: &mut Vec<usize>,
       enter_time: &mut Vec<usize>, curr_time: &mut usize, i: usize) -> Option<usize> {

    *curr_time += 1;
    enter_time[i] = *curr_time;
    subsz[i] = 1;

    for j in g[i].iter() {
        if *j == par[i] {
            continue;
        }
        if enter_time[*j] != 0 && enter_time[*j] < enter_time[i] {
            over_cnt[i] = over_cnt[i].wrapping_add(1);
            over_cnt[*j] = over_cnt[*j].wrapping_sub(1);
            continue;
        }
        if enter_time[*j] != 0 {
            continue;
        }

        par[*j] = i;
        if let Some(s) = dfs(g, subsz, over_cnt, par, enter_time, curr_time, *j) {
            return Some(s);
        }

        subsz[i] += subsz[*j];
        over_cnt[i] = over_cnt[i].wrapping_add(over_cnt[*j]);
    }

    if par[i] < g.len() {
        assert!(over_cnt[i] >= 2);
        if over_cnt[i] == 2 {
            return Some(subsz[i]);
        }
    }

    None
}

fn solve(input: Vec<String>) -> usize {
    let mut g: Vec<Vec<usize>> = vec![vec![]; INF];
    let mut mp: HashMap<String, usize> = HashMap::new();

    for line in input {
        let (left, right_list) = line.split_once(": ").unwrap();
        for right in right_list.split(' ') {
            let i = get_id(&mut mp, left);
            let j = get_id(&mut mp, right);
            g[i].push(j);
            g[j].push(i);
        }
    }
    while g.last().unwrap().is_empty() {
        g.pop();
    }
    let n = g.len();

    loop {
        let mut subsz = vec![0; n];
        let mut over_cnt = vec![0; n];
        let mut par = vec![INF; n];
        let mut enter_time = vec![0; n];
        let mut curr_time = 0;

        if let Some(sz) = dfs(&g, &mut subsz, &mut over_cnt, &mut par, &mut enter_time, &mut curr_time, 0) {
            return sz * (n - sz);
        }

        dbg!(g, subsz, over_cnt, par, enter_time, curr_time);

        panic!()
    }
}

fn main() {
    let ans = solve(io::stdin().lock().lines().flatten().collect());
    dbg!(ans);
}

#[test]
fn sample() {
    let txt = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    dbg!(ans);
    assert_eq!(ans, 54);
}
