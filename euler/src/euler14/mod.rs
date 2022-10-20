mod tests;

fn collatz(q: u64) -> i32 {
    let mut size = 1;
    let mut n = q;

    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        size += 1;
    }

    size
}

fn largest_collatz_sequence(under: u64) -> u64 {
    let mut largest = 1;
    let mut largest_size = 1;

    for q in 1..under {
        let size = collatz(q);

        if size > largest_size {
            largest = q;
            largest_size = size;
        }
    }

    largest
}

#[allow(dead_code)]
pub fn euler14() -> u64 {
    let result = largest_collatz_sequence(1_000_000);

    result
}
