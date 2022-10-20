#![cfg(test)]

use super::*;

#[test]
fn euler13_should_return_correct_result() {
    let result = euler13();

    let expected = 5537376230_u64;
    assert_eq!(result, expected);
}
