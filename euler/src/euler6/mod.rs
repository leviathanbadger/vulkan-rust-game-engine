mod tests;

fn sum_squares(mut n: i64) -> i64 {
    let mut sum = 0i64;

    while n > 0 {
        sum += n * n;
        n -= 1;
    }

    sum
}

fn square_sum(n: i64) -> i64 {
    let sum = (n * (n + 1)) / 2;
    return sum * sum;
}

#[allow(dead_code)]
pub fn euler6() -> i64 {
    let n = 100i64;
    let result = square_sum(n) - sum_squares(n);

    result
}
