#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler6_should_return_correct_result() {
    let result = euler6();

    let expected = 25164150i64;
    assert_eq!(result, expected);
}

test_cases!(sum_squares_should_return_correct_values, [
    (euler_sample, 10, 385)
], (n, expected) {
    let result = sum_squares(n);
    assert_eq!(result, expected);
});

test_cases!(square_sum_should_return_correct_values, [
    (euler_sample, 10, 3025)
], (n, expected) {
    let result = square_sum(n);
    assert_eq!(result, expected);
});
