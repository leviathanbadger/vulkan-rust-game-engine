#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler9_should_return_correct_result() {
    let result = euler9();

    let expected = 31875000;
    assert_eq!(result, expected);
}

test_cases!(product_adjacent_digits_should_return_correct_values, [
    (euler_sample, (3, 4, 5), true),
    (case_5_12_13, (5, 12, 13), true),
    (case_5_13_12, (5, 13, 12), false),
    (case_7_24_25, (7, 24, 25), true),
    (case_39_80_89, (39, 80, 89), true)
], (input, expected) {
    let (a, b, c) = input;
    let result = is_pythagorean_triplet(a, b, c);
    assert_eq!(result, expected);
});

test_cases!(find_pythagorean_triplet_with_valid_sum_should_return_correct_values, [
    (euler_sample, (3, 4, 5)),
    (case_5_12_13, (5, 12, 13)),
    (case_7_24_25, (7, 24, 25)),
    (case_39_80_89, (39, 80, 89))
], (expected) {
    let (a, b, c) = expected;
    let sum = a + b + c;
    let result = find_pythagorean_triplet_with_sum(sum);
    assert_eq!(result, Some(expected));
});

test_cases!(find_pythagorean_triplet_with_invalid_sum_should_return_correct_values, [
    (case_sum_3, 3)
], (sum) {
    let result = find_pythagorean_triplet_with_sum(sum);
    assert_eq!(result, None);
});
