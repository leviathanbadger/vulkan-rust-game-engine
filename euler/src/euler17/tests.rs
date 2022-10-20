#![cfg(test)]

use super::*;
use crate::test_cases;

#[test]
fn euler17_should_return_correct_result() {
    let result = euler17();

    let expected = 21124;
    assert_eq!(result, expected);
}

test_cases!(number_letter_count_should_return_correct_values, [
    (euler_sample_342, 342, 23),
    (euler_sample_115, 115, 20),
    (case_100, 100, "onehundred".len()),
    (case_1000, 1000, "onethousand".len())
], (n, expected) {
    let result = number_letter_count(n);
    assert_eq!(result, expected);
});
