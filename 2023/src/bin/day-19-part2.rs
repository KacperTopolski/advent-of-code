use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn trace(id: String, mp: &BTreeMap<String, Vec<String>>, mut nums: [[u128; 2]; 4]) -> u128 {
    // dbg!(&id, &nums);

    if nums.iter().any(|lr| lr[0] > lr[1]) || id == "R" {
        return 0;
    }
    if id == "A" {
        return nums.iter()
            .map(|lr| lr[1] - lr[0] + 1)
            .reduce(|a, b| a * b)
            .unwrap();
    }

    let mut agg = 0;

    for rule in mp[&id].iter() {
        if !rule.contains(':') {
            // dbg!("call from", &id);
            return agg + trace(rule.clone(), &mp, nums);
        }
        let (req, go) = rule.split_once(':').unwrap();
        let mut it = req.chars();

        let cmp_id = match it.next().unwrap() {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!()
        };
        let sgn = it.next().unwrap();
        let cmp_r = it.collect::<String>().parse::<u128>().unwrap();

        let mut numsrec = nums.clone();

        if sgn == '>' {
            numsrec[cmp_id][0] = numsrec[cmp_id][0].max(cmp_r + 1);
            nums[cmp_id][1] = nums[cmp_id][1].min(cmp_r);
        }
        else if sgn == '<' {
            numsrec[cmp_id][1] = numsrec[cmp_id][1].min(cmp_r - 1);
            nums[cmp_id][0] = nums[cmp_id][0].max(cmp_r);
        }
        else {
            panic!()
        }

        // dbg!("call from", &id);
        agg += trace(go.to_string(), &mp, numsrec);
    }

    panic!()
}

fn solve(input: Vec<String>) -> u128 {
    let mut rules: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for line in input {
        if line.is_empty() || line.starts_with('{') {
            continue;
        }
        let (name, line) = line.split_once('{').unwrap();
        let line = line.strip_suffix('}').unwrap().split(',').map(|s| s.to_string()).collect();
        rules.insert(name.to_string(), line);
    }

    let nums = [[1, 4000], [1, 4000], [1, 4000], [1, 4000]];

    dbg!(trace("in".to_string(), &rules, nums))
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
    assert_eq!(ans, 167409079868000);
}

#[test]
fn mine() {
    let txt = "\
in{x>2:R,m>1:R,a>2:R,s>1:R,A}";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    assert_eq!(ans, 4);
}
