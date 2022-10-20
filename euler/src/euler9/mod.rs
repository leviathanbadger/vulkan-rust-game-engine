mod tests;

fn is_pythagorean_triplet(a: i32, b: i32, c: i32) -> bool {
    return (a * a) + (b * b) == (c * c);
}

fn find_pythagorean_triplet_with_sum(sum: i32) -> Option<(i32, i32, i32)> {
    for c in 1..(sum - 2) {
        for a in 1..((sum + 1 - c) / 2) {
            let b = sum - (a + c);
            if is_pythagorean_triplet(a, b, c) {
                return Some((a, b, c));
            }
        }
    }

    None
}

#[allow(dead_code)]
pub fn euler9() -> i32 {
    let triplet_opt = find_pythagorean_triplet_with_sum(1000);

    match triplet_opt {
        Some(triplet) => {
            let (a, b, c) = triplet;
            a * b * c
        },
        None => 0
    }
}
