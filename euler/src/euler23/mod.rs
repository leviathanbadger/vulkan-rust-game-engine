mod tests;

use crate::util::{sum_proper_divisors};

fn is_abundant_number(n: i32) -> bool {
    sum_proper_divisors(n) > n
}

fn get_abundant_numbers_below(below: i32) -> Vec<i32> {
    let mut abundant_numbers = vec![0i32; 0];

    for q in 12..below {
        if is_abundant_number(q) {
            abundant_numbers.push(q);
        }
    }

    abundant_numbers
}

fn get_non_abundant_sums() -> Vec<i32> {
    const MAX_THRESHOLD: usize = 28123;
    let abundant_numbers = get_abundant_numbers_below(MAX_THRESHOLD as i32);

    let mut is_abundant_sum = [false; MAX_THRESHOLD];

    let abundant_number_count = abundant_numbers.len();
    for q in 0..abundant_number_count {
        for w in 0..(q + 1) {
            let sum = abundant_numbers[q] + abundant_numbers[w];
            if sum < MAX_THRESHOLD as i32 {
                is_abundant_sum[sum as usize] = true;
            }
            else { break; }
        }
    }

    (1..MAX_THRESHOLD)
        .filter(|&q| !is_abundant_sum[q])
        .map(|q| { q as i32 })
        .collect()
}

#[allow(dead_code)]
pub fn euler23() -> i32 {
    let non_abundant_sums = get_non_abundant_sums();

    let mut sum = 0;

    for n in non_abundant_sums {
        sum += n;
    }

    sum
}
