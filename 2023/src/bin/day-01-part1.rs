use std::io;
use std::io::prelude::*;

fn find_first_digit<I: std::iter::Iterator<Item = char>>(iter: &mut I) -> u32 {
    for ch in iter {
        if ch.is_digit(10) {
            return ch.to_digit(10).unwrap();
        }
    }
    panic!();
}

fn main() {
    let mut ans = 0;
    for line in io::stdin().lock().lines().map(|x| x.unwrap()) {
        let first = find_first_digit(&mut line.chars());
        let last = find_first_digit(&mut line.chars().rev());
        ans += first * 10 + last;
    }
    dbg!(ans);
}
