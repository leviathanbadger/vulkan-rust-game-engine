#![cfg(test)]

use super::*;

#[test]
fn euler67_should_return_correct_result() {
    let result = euler67();

    let expected = 7273;
    assert_eq!(result, expected);
}
