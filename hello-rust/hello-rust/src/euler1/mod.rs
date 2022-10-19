mod tests;

#[allow(dead_code)]
fn sum_multiples(below: i32, multiple_of: i32) -> i32 {
    let num_multiples = (below - 1) / multiple_of;

    ((num_multiples * (num_multiples + 1)) / 2) * multiple_of
}

macro_rules! sum_multiples {
    ( $below:expr , $multiple_of:expr ) => {
        {
            let below = { $below };
            let multiple_of = { $multiple_of };
            let num_multiples = (below - 1) / multiple_of;
            ((num_multiples * (num_multiples + 1)) / 2) * multiple_of
        }
    };
}

#[allow(dead_code)]
pub const fn euler1() -> i32 {
    const UP_TO: i32 = 1000;

    const RESULT: i32 = sum_multiples!(UP_TO, 3) + sum_multiples!(UP_TO, 5) - sum_multiples!(UP_TO, 15);

    RESULT
}
