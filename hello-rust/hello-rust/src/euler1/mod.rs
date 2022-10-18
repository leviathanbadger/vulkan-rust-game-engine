

fn sum_multiples(below: i32, multiple_of: i32) -> i32 {
    let num_multiples = (below - 1) / multiple_of;
    return ((num_multiples * (num_multiples + 1)) / 2) * multiple_of;
}

#[allow(dead_code)]
pub fn euler1() {
    let up_to = 1000;
    let sum = sum_multiples(up_to, 3) + sum_multiples(up_to, 5) - sum_multiples(up_to, 15);
    println!("Euler 1: {}", sum);
}
