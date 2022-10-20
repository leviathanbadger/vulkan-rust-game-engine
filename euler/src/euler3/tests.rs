#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler3_should_return_correct_result() {
    let result = euler3();

    let expected = 6857;
    assert_eq!(result, expected);
}

test_cases!(largest_prime_factor_should_return_correct_values, [
    (euler_sample, 13195, 29)
], (idx, expected) {
    let result = largest_prime_factor(idx);
    assert_eq!(result, expected);
});
