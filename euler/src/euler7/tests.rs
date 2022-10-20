#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler7_should_return_correct_result() {
    let result = euler7();

    let expected = 104743;
    assert_eq!(result, expected);
}

test_cases!(nth_prime_should_return_correct_values, [
    (euler_sample, 5, 13)
], (n, expected) {
    let result = nth_prime(n);
    assert_eq!(result, expected);
});
