#![cfg(test)]

use super::*;
use crate::test_cases;
use crate::tree_path;

#[test]
fn euler18_should_return_correct_result() {
    let result = euler18();

    let expected = 1074;
    assert_eq!(result, expected);
}

fn sample_tree() -> [Vec<i32>; 4] {
    tree_path!(
        3;
        7 4;
        2 4 6;
        8 5 9 3
    )
}

test_cases!(minimum_path_sum_should_return_correct_values, [
    (euler_sample, sample_tree(), 23)
], (tree, expected) {
    let result = minimum_path_sum(tree);
    assert_eq!(result, expected);
});
