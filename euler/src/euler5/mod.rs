mod tests;

fn smallest_multiple(n: i32) -> i64 {
    assert!(n <= 20);

    let mut smallest = 1i64;
    let mut x = 2i64;

    while x <= n as i64 {
        if smallest % x != 0 {
            let mut factor = 2;
            loop {
                if (smallest * factor) % x == 0 {
                    break;
                }
                factor += 1;
            }
            smallest *= factor;
        }
        x += 1;
    }

    smallest
}

#[allow(dead_code)]
pub fn euler5() -> i64 {
    let result = smallest_multiple(20);

    result
}
