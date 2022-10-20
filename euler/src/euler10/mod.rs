mod tests;

use crate::util::{prime_sieve};

fn sum_primes_below(below: i32) -> i64 {
    let primes = prime_sieve(below as usize);

    let mut sum = 0i64;
    for prime in primes {
        sum += prime as i64;
    }

    sum
}

#[allow(dead_code)]
pub fn euler10() -> i64 {
    let result = sum_primes_below(2_000_000);

    result
}
