#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler14_should_return_correct_result() {
    let result = euler14();

    let expected = 837799;
    assert_eq!(result, expected);
}

test_cases!(collatz_should_return_correct_values, [
    (euler_sample, 13, 10)
], (n, expected) {
    let result = collatz(n);
    assert_eq!(result, expected);
});
