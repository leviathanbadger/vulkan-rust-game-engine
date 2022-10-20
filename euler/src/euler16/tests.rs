#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler16_should_return_correct_result() {
    let result = euler16();

    let expected = 1366;
    assert_eq!(result, expected);
}

test_cases!(sum_pow_digits_should_return_correct_values, [
    (euler_sample, (2, 15), 26)
], (input, expected) {
    let (n, pow) = input;
    let result = sum_pow_digits(n, pow);
    assert_eq!(result, expected);
});
