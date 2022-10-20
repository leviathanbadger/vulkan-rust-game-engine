#![cfg(test)]

use super::*;

#[test]
fn euler11_should_return_correct_result() {
    let result = euler11();

    let expected = 70600674;
    assert_eq!(result, expected);
}
