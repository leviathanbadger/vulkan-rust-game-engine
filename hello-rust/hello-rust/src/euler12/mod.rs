mod tests;

struct TriangleNumber {
    curr: i32,
    add_next: i32
}

impl Iterator for TriangleNumber {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr += self.add_next;
        self.add_next += 1;

        Some(self.curr)
    }
}

fn triangle_numbers() -> TriangleNumber {
    TriangleNumber { curr: 0, add_next: 1 }
}

fn divisor_count(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    let mut count = 2;

    let sqrt = f32::ceil(f32::sqrt(n as f32)) as i32;
    for q in 2..sqrt {
        if n % q == 0 {
            count += 2;
        }
    }

    if sqrt * sqrt == n {
        count += 1
    }

    count
}

fn first_triangle_num_with_divisor_count(count: i32) -> Option<i32> {
    for n in triangle_numbers() {
        let d_count = divisor_count(n);
        if d_count >= count {
            return Some(n);
        }
    }

    None
}

#[allow(dead_code)]
pub fn euler12() -> i32 {
    let result = first_triangle_num_with_divisor_count(500);

    result.unwrap_or(-1)
}
