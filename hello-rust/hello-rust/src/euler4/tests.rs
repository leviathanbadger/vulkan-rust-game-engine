#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler4_should_return_correct_result() {
    let result = euler4();

    let expected = 906609;
    assert_eq!(result, expected);
}

test_cases!(is_palindromic_should_return_correct_values, [
    (euler_sample, [9, 0, 0, 9], true),
    (hump_4, [1, 2, 2, 1], true),
    (hump_5, [1, 2, 3, 2, 1], true),
    (ramp_3, [1, 2, 3], false),
    (ramp_2, [1, 2], false),
    (solo, [1], true),
    (zero, [], true)
], (input, expected) {
    let result = is_palindromic(&input[..]);
    assert_eq!(result, expected);
});

test_cases!(largest_palindrome_product_should_return_correct_values, [
    (euler_sample, (10, 99), 9009)
], (input, expected) {
    let (min, max) = input;
    let result = largest_palindrome_product(min, max);
    assert_eq!(result, expected);
});
