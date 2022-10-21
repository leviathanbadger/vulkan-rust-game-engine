#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler22_should_return_correct_result() {
    let result = euler22();

    let expected = 871198282;
    assert_eq!(result, expected);
}

test_cases!(name_score_should_return_correct_values, [
    (euler_sample, (&"COLIN", 937), 49714)
], (input, expected) {
    let (name, idx) = input;

    let result = name_score(name, idx);
    assert_eq!(result, expected);
});
