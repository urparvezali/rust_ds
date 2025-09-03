#![allow(unused)]

pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for p in 2..=n.isqrt() {
        while n % p == 0 {
            n /= p;
            factors.push(p);
        }
    }
    factors
}

pub fn phi(mut n: u64) -> u64 {
    let mut res = n;
    for p in 2..=n.isqrt() {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            res -= res / p;
        }
    }
    if n > 1 {
        res -= res / n;
    }
    res
}
