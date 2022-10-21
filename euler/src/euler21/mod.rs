mod tests;

use crate::util::{sum_proper_divisors};

fn get_amicable_number_pair(a: i32) -> Option<(i32, i32)> {
    let b = sum_proper_divisors(a);
    if b != a && sum_proper_divisors(b) == a {
        Some((i32::min(a, b), i32::max(a, b)))
    }
    else {
        None
    }
}

fn get_amicable_numbers_below(below: i32) -> Vec<i32> {
    let mut amicable_numbers = Vec::new();

    for q in 2..=below {
        let pair = get_amicable_number_pair(q);
        match pair {
            None => { }
            Some((a, b)) => {
                if a == q {
                    amicable_numbers.push(a);
                    if b < below {
                        amicable_numbers.push(b);
                    }
                }
            }
        }
    }

    amicable_numbers
}

#[allow(dead_code)]
pub fn euler21() -> i32 {
    let amicable_numbers = get_amicable_numbers_below(10000);

    let mut sum = 0;

    for n in amicable_numbers {
        sum += n;
    }

    sum
}
