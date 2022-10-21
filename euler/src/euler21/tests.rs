#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler21_should_return_correct_result() {
    let result = euler21();

    let expected = 31626;
    assert_eq!(result, expected);
}

test_cases!(get_amicable_number_pair_should_return_correct_values_valid, [
    (euler_sample, (220, 284))
], (pair) {
    let (a, b) = pair;

    let result1 = get_amicable_number_pair(a);
    assert_eq!(result1, Some(pair));

    let result2 = get_amicable_number_pair(b);
    assert_eq!(result2, Some(pair));
});

test_cases!(get_amicable_number_pair_should_return_correct_values_invalid, [
    (a_equals_b, 6)
], (a) {
    let result1 = get_amicable_number_pair(a);
    assert_eq!(result1, None);
});
