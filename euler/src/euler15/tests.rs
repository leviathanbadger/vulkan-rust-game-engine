#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler15_should_return_correct_result() {
    let result = euler15();

    let expected = 137846528820u64;
    assert_eq!(result, expected);
}

test_cases!(lattice_path_count_should_return_correct_values, [
    (euler_sample, 2, 6)
], (n, expected) {
    let result = lattice_path_count(n);
    assert_eq!(result, expected);
});
