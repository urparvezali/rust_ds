#![allow(unused)]
pub struct Sieve {
    prime: Vec<bool>,
}

impl Sieve {
    pub fn new(limit: usize) -> Self {
        let mut prime = vec![true; limit + 1];
        prime[0] = false;
        prime[1] = false;

        for i in 2..=limit.isqrt() {
            if prime[i] {
                if i * i > limit {
                    break;
                }
                for j in (i * i..=limit).step_by(i) {
                    prime[j] = false;
                }
            }
        }

        Self { prime }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        if n < self.prime.len() {
            self.prime[n]
        } else {
            false
        }
    }

    pub fn primes(&self) -> Vec<usize> {
        self.prime
            .iter()
            .enumerate()
            .filter_map(|(i, &is_p)| if is_p { Some(i) } else { None })
            .collect()
    }
}

pub struct SegmentedSieve {
    l: usize,
    r: usize,
    v: Vec<bool>,
}

impl SegmentedSieve {
    pub fn new(l: usize, r: usize) -> Self {
        let mut v = vec![true; r - l + 1];

        if l == 0 {
            v[0] = false;
            if r >= 1 {
                v[1] = false;
            }
        } else if l == 1 {
            v[0] = false;
        }

        let base_primes = Sieve::new(r.isqrt()).primes();

        for &p in &base_primes {
            let mut start = (l + p - 1) / p * p;
            if start < p * p {
                start = p * p;
            }

            for j in (start..=r).step_by(p) {
                v[j - l] = false;
            }
        }

        Self { l, r, v }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        if n < self.l || n > self.r {
            panic!("Number {} not in sieve range [{}, {}]", n, self.l, self.r);
        }
        self.v[n - self.l]
    }

    pub fn primes(&self) -> Vec<usize> {
        self.v
            .iter()
            .enumerate()
            .filter_map(|(i, &is_p)| if is_p { Some(i + self.l) } else { None })
            .collect()
    }
}
