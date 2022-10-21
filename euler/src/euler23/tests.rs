#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler23_should_return_correct_result() {
    let result = euler23();

    let expected = 4179871;
    assert_eq!(result, expected);
}

test_cases!(is_abundant_number_should_return_correct_values, [
    (euler_sample, 12, true),
    (case_11, 11, false)
], (n, expected) {
    let result = is_abundant_number(n);
    assert_eq!(result, expected);
});
