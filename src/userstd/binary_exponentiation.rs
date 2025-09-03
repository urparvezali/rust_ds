#![allow(unused)]

pub fn bin_exp(x: i64, n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    let half = bin_exp(x, n / 2);
    if n & 1 == 1 {
        return x * half * half;
    }
    half * half
}

pub fn bin_exp_mod(mut x: i64, n: i64, m: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    x %= m;
    let half = bin_exp_mod(x, n / 2, m);
    if n & 1 == 1 {
        return ((x * half) % m) % m;
    }
    (half * half) % m
}
