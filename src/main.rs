use std::time::Instant;

use crate::userstd::sieve::SegmentedSieve;

mod userstd;

fn main() {
    let time = Instant::now();
    let ss = SegmentedSieve::new(5, 10000000);
    println!("{:?}", ss.is_prime(500003));
    println!("{:?}", time.elapsed());
}
