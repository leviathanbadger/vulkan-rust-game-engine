#![cfg(test)]

use super::*;
use crate::test_cases;

use crate::util::{prime_sieve};

#[test]
fn euler27_should_return_correct_result() {
    let result = euler27();

    let expected = -59231;
    assert_eq!(result, expected);
}

test_cases!(is_abundant_number_should_return_correct_values, [
    (euler_sample_41, (1, 41), 40),
    (euler_sample_79, (-79, 1601), 80)
], (input, expected) {
    let (a, b) = input;
    let primes_vec = prime_sieve(((expected + a) * expected + b + 1) as usize);
    let primes = HashSet::from_iter(primes_vec);
    let result = consecutive_quadratic_primes(a, b, &primes);
    assert_eq!(result, expected);
});
