use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn get_num(s: &str) -> usize {
    s.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap()
}

fn trace(id: String, x: usize, m: usize, a: usize, s: usize, mp: &BTreeMap<String, Vec<String>>) -> usize {
    dbg!(&id, x, m, a, s);

    if id == "A" {
        return x + m + a + s;
    }
    if id == "R" {
        return 0;
    }

    for rule in mp[&id].iter() {
        if !rule.contains(':') {
            return trace(rule.clone(), x, m, a, s, &mp);
        }
        let (req, go) = rule.split_once(':').unwrap();
        let mut it = req.chars();

        let cmp_l = match it.next().unwrap() {
            'x' => x,
            'm' => m,
            'a' => a,
            's' => s,
            _ => panic!()
        };
        let sgn = it.next().unwrap();
        let cmr_r = it.collect::<String>().parse::<usize>().unwrap();

        if match sgn {
            '>' => cmp_l > cmr_r,
            '=' => cmp_l == cmr_r,
            '<' => cmp_l < cmr_r,
            _ => panic!()
        } {
            return trace(go.to_string(), x, m, a, s, &mp);
        }
    }

    panic!()
}

fn solve(input: Vec<String>) -> usize {
    let mut ans = 0;
    let mut rules: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for line in input {
        if line.is_empty() {
            continue;
        }
        if line.starts_with('{') {
            let mut xmas = line.split(',').map(get_num);
            ans += trace("in".to_string(), xmas.next().unwrap(), xmas.next().unwrap(), xmas.next().unwrap(), xmas.next().unwrap(), &rules);
            continue;
        }
        let (name, line) = line.split_once('{').unwrap();
        let line = line.strip_suffix('}').unwrap().split(',').map(|s| s.to_string()).collect();
        rules.insert(name.to_string(), line);
    }

    dbg!(&rules);

    dbg!(ans)
}

fn main() {
    solve(io::stdin().lock().lines().flatten().collect());
}

#[test]
fn sample() {
    let txt = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 19114);
}
