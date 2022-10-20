use num_bigint::BigUint;

mod tests;

fn sum_pow_digits(n: i32, pow: i32) -> i32 {
    let n_biguint = BigUint::from(n as u128);

    let mut product = BigUint::from(1u128);
    for _q in 0..pow {
        product *= &n_biguint;
    }

    let product_str = product.to_str_radix(10);

    let mut sum = 0;
    for chr in product_str.chars() {
        sum += (chr as i32) - ('0' as i32);
    }

    sum
}

#[allow(dead_code)]
pub fn euler16() -> i32 {
    let result = sum_pow_digits(2, 1000);

    result
}
