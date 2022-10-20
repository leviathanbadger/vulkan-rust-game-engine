mod tests;

use num_bigint::BigUint;

fn factorial_digit_sum(n: i32) -> i32 {
    let mut product = BigUint::from(1_u128);

    for q in 2..=n {
        product *= BigUint::from(q as u128);
    }

    let product_str = product.to_str_radix(10);

    let mut sum = 0;

    for chr in product_str.chars() {
        sum += (chr as i32) - ('0' as i32);
    }

    sum
}

#[allow(dead_code)]
pub fn euler20() -> i32 {
    let result = factorial_digit_sum(100);

    result
}
