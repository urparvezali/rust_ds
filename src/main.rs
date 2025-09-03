use crate::userstd::factorization::prime_factors;

mod userstd;

fn main() {
    let primes = prime_factors(1239587894);
    println!("{:?}", primes);
    println!("{}", primes.iter().fold(1, |acc, x| acc * x));
}
