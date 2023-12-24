use std::io;
use std::io::prelude::*;

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    if a > b {
        return gcd(b, a);
    }
    match a {
        0 => b,
        _ => gcd(b % a, a)
    }
}

type Ray = (i128, i128, i128, i128);

fn parse(txt: &str) -> Ray {
    let (left, right) = txt.split_once('@').unwrap();
    let nums_left: Vec<i128> = left.split(',').map(|s| s.trim().parse::<i128>()).flatten().collect();
    let nums_right: Vec<i128> = right.split(',').map(|s| s.trim().parse::<i128>()).flatten().collect();
    assert_eq!(nums_left.len(), 3);
    assert_eq!(nums_right.len(), 3);

    let g = gcd(nums_right[0], nums_right[1]);

    (nums_left[0], nums_left[1], nums_right[0] / g, nums_right[1] / g)
}

fn det(a: i128, b: i128, c: i128, d: i128) -> i128 {
    a * d - b * c
}

fn check(init: i128, delta: i128, mut num_t: i128, mut den_t: i128, mn: i128, mx: i128) -> bool {
    if den_t < 0 {
        den_t = -den_t;
        num_t = -num_t;
    }
    if num_t < 0 {
        return false;
    }

    let mn_b = mn * den_t <= init * den_t + delta * num_t;
    let mx_b = init * den_t + delta * num_t <= mx * den_t;

    mn_b && mx_b
}

fn sol_ray(a: &Ray, b: &Ray, mn: i128, mx: i128) -> bool {
    let (x1, y1, dx1, dy1) = a.clone();
    let (x2, y2, dx2, dy2) = b.clone();
    // dbg!(a, b);

    let num_t = det(x2 - x1, -dx2, y2 - y1, -dy2);
    let num_u = det(dx1, x2 - x1, dy1, y2 - y1);
    let den = det(dx1, -dx2, dy1, -dy2);

    if den == 0 {
        if (x1 - x2) * dy1 != (y1 - y2) * dx1 {
            return false;
        }
        panic!()
    }

    let cx1 = check(x1, dx1, num_t, den, mn, mx);
    let cy1 = check(y1, dy1, num_t, den, mn, mx);
    let cx2 = check(x2, dx2, num_u, den, mn, mx);
    let cy2 = check(y2, dy2, num_u, den, mn, mx);

    cx1 && cy1 && cx2 && cy2
}

fn solve(input: Vec<String>, mn: i128, mx: i128) -> usize {
    let rays: Vec<Ray> = input.iter().map(|s| parse(s)).collect();
    dbg!(&rays);

    let mut ans = 0;

    for (i, ri) in rays.iter().enumerate() {
        for rj in rays.iter().take(i) {
            ans += sol_ray(ri, rj, mn, mx) as usize;
        }
    }

    ans
}

fn main() {
    let ans = solve(io::stdin().lock().lines().flatten().collect(), 200000000000000, 400000000000000);
    dbg!(ans);
}

#[test]
fn sample() {
    let txt = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect(), 7, 27);
    dbg!(ans);
    assert_eq!(ans, 2);
}
