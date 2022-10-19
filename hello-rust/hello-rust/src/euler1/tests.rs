#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler1_should_return_correct_result() {
    let result = euler1();

    let expected = 233168;
    assert_eq!(result, expected);
}

test_cases!(sum_multiples_should_return_correct_value, [
    (multiples_3_in_10, (10, 3), 18),
    (multiples_5_in_10, (10, 5), 5)
], (input, expected) {
    let result = sum_multiples(input.0, input.1);
    assert_eq!(result, expected);
});
