#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler10_should_return_correct_result() {
    let result = euler10();

    let expected = 142913828922i64;
    assert_eq!(result, expected);
}

test_cases!(sum_primes_below_should_return_correct_values, [
    (euler_sample, 10, 17),
    (exclude_below, 7, 10)
], (below, expected) {
    let result = sum_primes_below(below);
    assert_eq!(result, expected);
});
