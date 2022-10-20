#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler20_should_return_correct_result() {
    let result = euler20();

    let expected = 648;
    assert_eq!(result, expected);
}

test_cases!(minimum_path_sum_should_return_correct_values, [
    (euler_sample, 10, 27)
], (tree, expected) {
    let result = factorial_digit_sum(tree);
    assert_eq!(result, expected);
});
