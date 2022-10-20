#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler8_should_return_correct_result() {
    let result = euler8();

    let expected = 23514624000i64;
    assert_eq!(result, expected);
}

test_cases!(product_adjacent_digits_should_return_correct_values, [
    (euler_sample, 4, 5832i64)
], (num_digits, expected) {
    let result = product_adjacent_digits(num_digits, TEST_AGAINST);
    assert_eq!(result, expected);
});
