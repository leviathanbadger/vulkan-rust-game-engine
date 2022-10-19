#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler2_should_return_correct_result() {
    let result = euler2();

    let expected = 4613732;
    assert_eq!(result, expected);
}

test_cases!(fib_should_return_correct_values, [
    (idx_0, 0, 1),
    (idx_3, 3, 3),
    (idx_5, 5, 8)
], (idx, expected) {
    let result = fib(idx);
    assert_eq!(result, expected);
});
