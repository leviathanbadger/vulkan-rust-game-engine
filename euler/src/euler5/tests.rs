#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler5_should_return_correct_result() {
    let result = euler5();

    let expected = 232792560i64;
    assert_eq!(result, expected);
}

test_cases!(smallest_multiple_should_return_correct_values, [
    (euler_sample, 10, 2520)
], (n, expected) {
    let result = smallest_multiple(n);
    assert_eq!(result, expected);
});
