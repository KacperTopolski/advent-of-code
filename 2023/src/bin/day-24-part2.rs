use std::io;
use std::io::prelude::*;

type Ray = (i128, i128, i128, i128);

const MOD: i128 = 1_000000_000000_000000 + 31;

fn parse(txt: &str, id1: usize, id2: usize) -> Ray {
    let (left, right) = txt.split_once('@').unwrap();
    let nums_left: Vec<i128> = left.split(',').map(|s| s.trim().parse::<i128>()).flatten().collect();
    let nums_right: Vec<i128> = right.split(',').map(|s| s.trim().parse::<i128>()).flatten().collect();
    assert_eq!(nums_left.len(), 3);
    assert_eq!(nums_right.len(), 3);

    (nums_left[id1], nums_left[id2], nums_right[id1], nums_right[id2])
}

fn sol_ray(a: &Ray, b: &Ray) -> [i128; 5] {
    let (xi, yi, dxi, dyi) = a.clone();
    let (xj, yj, dxj, dyj) = b.clone();

    [
        -dyi + dyj,
        dxi - dxj,
        yi - yj,
        -xi + xj,
        yi * dxi - xi * dyi - yj * dxj + xj * dyj
    ]
}

fn det_4(mat: &[[i128; 4]; 4]) -> i128 {
    let mut det = 0;

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                for l in 0..4 {
                    if i == j || i == k || i == l || j == k || j == l || k == l {
                        continue;
                    }
                    let here = mat[0][i] * mat[1][j] % MOD * mat[2][k] % MOD * mat[3][l] % MOD;
                    if (i < j) ^ (i < k) ^ (i < l) ^ (j < k) ^ (j < l) ^ (k < l) {
                        det -= here;
                    }
                    else {
                        det += here;
                    }
                }
            }
        }
    }

    det %= MOD;
    if det < 0 {
        det += MOD;
    }
    det
}

fn inv(value: i128) -> i128 {
    let mut base = value;
    let mut exp = MOD - 2;
    let mut ans = 1;

    while exp > 0 {
        if (exp & 1) > 0 {
            ans *= base;
            ans %= MOD;
        }
        base *= base;
        base %= MOD;
        exp /= 2;
    }

    assert_eq!(ans * value % MOD, 1);

    ans
}

fn sol(mat: [[i128; 5]; 4]) -> [i128; 4] {
    let mut ans: [i128; 4] = [0, 0, 0, 0];
    let mut tmp = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    dbg!(&mat);

    for i in 0..4 {
        for j in 0..4 {
            tmp[i][j] = mat[i][j];
        }
    }
    let den = det_4(&tmp);
    for k in 0..4 {
        for i in 0..4 {
            for j in 0..4 {
                tmp[i][j] = if j != k { mat[i][j] } else { mat[i][4] };
            }
        }
        let num = det_4(&tmp);
        dbg!(num, den);
        ans[k] = num * inv(den) % MOD;

        assert_eq!(den * inv(den) % MOD, 1);

        if ans[k] > MOD / 2 {
            ans[k] -= MOD;
        }
    }

    dbg!(ans)
}

fn solve(input: Vec<String>) -> i128 {
    let rays_xy: Vec<Ray> = input.iter().map(|s| parse(s, 0, 1)).collect();
    // dbg!(&rays_xy);

    let mat = [
        sol_ray(&rays_xy[0], &rays_xy[1]),
        sol_ray(&rays_xy[1], &rays_xy[2]),
        sol_ray(&rays_xy[2], &rays_xy[3]),
        sol_ray(&rays_xy[3], &rays_xy[4])
    ];
    let xy_sol = sol(mat);

    let rays_xz: Vec<Ray> = input.iter().map(|s| parse(s, 0, 2)).collect();
    // dbg!(&rays_xz);

    let mat = [
        sol_ray(&rays_xz[0], &rays_xz[1]),
        sol_ray(&rays_xz[1], &rays_xz[2]),
        sol_ray(&rays_xz[2], &rays_xz[3]),
        sol_ray(&rays_xz[3], &rays_xz[4])
    ];
    let xz_sol = sol(mat);

    assert_eq!(xy_sol[0], xz_sol[0]);
    assert_eq!(xy_sol[2], xz_sol[2]);

    xy_sol[0] + xy_sol[1] + xz_sol[1]
}

fn main() {
    let ans = solve(io::stdin().lock().lines().flatten().collect());
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
    let ans = solve(txt.split('\n').map(|x| x.to_string()).collect());
    dbg!(ans);
    assert_eq!(ans, 47);
}
