

pub fn sum_proper_divisors(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }

    let mut sum = 1;

    let sqrt = f32::ceil(f32::sqrt(n as f32)) as i32;
    for q in 2..sqrt {
        if n % q == 0 {
            sum += q + (n / q);
        }
    }

    if sqrt * sqrt == n {
        sum += sqrt;
    }

    sum
}

mod tests {
    #![cfg(test)]

    use super::*;
    use crate::test_cases;

    test_cases!(sum_proper_divisors_should_return_correct_values, [
        (case_220, 220, 284),
        (case_284, 284, 220)
    ], (a, expected) {
        let result = sum_proper_divisors(a);
        assert_eq!(result, expected);
    });
}
