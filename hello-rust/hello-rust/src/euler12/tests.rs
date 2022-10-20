#![cfg(test)]

use super::*;
use crate::test_cases;
use crate::assert_iter_eq;

#[test]
fn euler12_should_return_correct_result() {
    let result = euler12();

    let expected = 76576500;
    assert_eq!(result, expected);
}

#[test]
fn triangle_numbers_should_return_correct_results() {
    let results = triangle_numbers().take(10);

    let expected = [1, 3, 6, 10, 15, 21, 28, 36, 45, 55];
    assert_iter_eq!(results, expected);
}

test_cases!(divisor_count_should_return_correct_values, [
    (case_1, 1, 1),
    (case_3, 3, 2),
    (case_6, 6, 4),
    (case_10, 10, 4),
    (case_15, 15, 4),
    (case_21, 21, 4),
    (case_28, 28, 6),
    (case_25, 25, 3)
], (n, expected) {
    let result = divisor_count(n);
    assert_eq!(result, expected);
});

test_cases!(first_triangle_num_with_divisor_count_should_return_correct_values, [
    (euler_sample, 5, 28)
], (divisor_count, expected) {
    let result = first_triangle_num_with_divisor_count(divisor_count).unwrap_or(-1);
    assert_eq!(result, expected);
});
