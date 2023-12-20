use std::collections::{HashMap, VecDeque};
use std::io;
use std::io::prelude::*;

const ITERS: usize = 1000;
fn solve(input: Vec<String>) -> usize {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut rgraph: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let mut state: HashMap<String, bool> = HashMap::new();
    let mut ntype: HashMap<String, char> = HashMap::new();

    for line in input {
        let (name, out) = line.split_once(" -> ").unwrap();
        let name_type = name.chars().next().unwrap();
        let name = name.trim_start_matches('%').trim_start_matches('&');
        let outs: Vec<_> = out.split(", ").map(|s| s.to_string()).collect();

        for out in outs.iter() {
            if !rgraph.contains_key(out) {
                rgraph.insert(out.to_string(), HashMap::new());
            }
            rgraph.get_mut(out).unwrap().insert(name.to_string(), false);
        }

        ntype.insert(name.to_string(), name_type);
        graph.insert(name.to_string(), outs);
        state.insert(name.to_string(), false);
    }

    dbg!(&graph, &rgraph, &state, &ntype);

    let mut counts = [0, 0];

    for _ in 0..ITERS {
        let mut signals: VecDeque<(String, bool, String)> = VecDeque::new();
        signals.push_back(("".to_string(), false, "broadcaster".to_string()));

        while let Some((from, type_here, to)) = signals.pop_front() {
            // dbg!(&from, type_here, &to, type_here as usize);
            counts[type_here as usize] += 1;
            let to_send: Option<bool> = match ntype.get(&to).cloned().unwrap_or('u') {
                'b' => Some(type_here),
                '%' => {
                    if type_here == false {
                        let state_here = state.get_mut(&to).unwrap();
                        *state_here ^= true;
                        Some(*state_here)
                    }
                    else {
                        None
                    }
                }
                '&' => {
                    let to_values = rgraph.get_mut(&to).unwrap();
                    to_values.insert(from.to_string(), type_here);
                    Some(to_values.values().all(|s| *s) ^ true)
                },
                'u' => { None },
                _ => panic!()
            };
            if let Some(sign) = to_send {
                for out in graph[&to].iter() {
                    signals.push_back((to.to_string(), sign, out.to_string()));
                }
            }
        }
    }

    dbg!(counts[0]) * dbg!(counts[1])
}

fn main() {
    let ans = solve(io::stdin().lock().lines().flatten().collect());
    dbg!(ans);
}

#[test]
fn sample_1() {
    let txt = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    dbg!(ans);
    assert_eq!(ans, 32000000);
}

#[test]
fn sample_2() {
    let txt = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    dbg!(ans);
    assert_eq!(ans, 11687500);
}
