use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::io;
use std::io::prelude::*;

fn gcd(a: u128, b: u128) -> u128 {
    if a > b {
        return gcd(b, a);
    }
    match a {
        0 => b,
        _ => gcd(b % a, a)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    a / gcd(a, b) * b
}

fn solve(input: Vec<String>) -> u128 {
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

    let mut signals: VecDeque<(String, bool, String)> = VecDeque::new();
    signals.push_back(("".to_string(), false, "broadcaster".to_string()));

    let mut to_guess: HashMap<(String, bool), Vec<i32>> = HashMap::new();

    for press_id in 1..50000 {
        let mut signals: VecDeque<(String, bool, String)> = VecDeque::new();
        signals.push_back(("".to_string(), false, "broadcaster".to_string()));

        while let Some((from, type_here, to)) = signals.pop_front() {
            if to == "rm" && type_here == true {
                let key = (from.to_string(), type_here);
                if !to_guess.contains_key(&key) {
                    to_guess.insert(key.clone(), vec![]);
                }
                to_guess.get_mut(&key).unwrap().push(press_id);
            }

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

    to_guess.values().map(|v| v[0] as u128).reduce(|a, b| lcm(a, b)).unwrap()
}

fn main() {
    let ans = solve(io::stdin().lock().lines().flatten().collect());
    dbg!(ans);
}
