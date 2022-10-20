mod tests;

fn number_letter_count(n: i32) -> usize {
    match n {
        1 => "one".len(),
        2 => "two".len(),
        3 => "three".len(),
        4 => "four".len(),
        5 => "five".len(),
        6 => "six".len(),
        7 => "seven".len(),
        8 => "eight".len(),
        9 => "nine".len(),
        10 => "ten".len(),
        11 => "eleven".len(),
        12 => "twelve".len(),
        13 => "thirteen".len(),
        14 => "fourteen".len(),
        15 => "fifteen".len(),
        16 => "sixteen".len(),
        17 => "seventeen".len(),
        18 => "eighteen".len(),
        19 => "nineteen".len(),
        20..=99 => {
            let tens_place = n / 10;
            let ones_place = n % 10;
            let count = if ones_place == 0 { 0 } else { number_letter_count(ones_place) };
            count + match tens_place {
                2 => "twenty".len(),
                3 => "thirty".len(),
                4 => "forty".len(),
                5 => "fifty".len(),
                6 => "sixty".len(),
                7 => "seventy".len(),
                8 => "eighty".len(),
                9 => "ninety".len(),
                _ => panic!("Not possible")
            }
        },
        100..=999 => {
            let hundreds_place = n / 100;
            let sub_hundred = n % 100;
            let count = if sub_hundred == 0 { 0 } else { "and".len() + number_letter_count(sub_hundred) };
            count + number_letter_count(hundreds_place) + "hundred".len()
        },
        1000 => "onethousand".len(),
        _ => panic!("Not supported")
    }
}

#[allow(dead_code)]
pub fn euler17() -> i32 {
    let mut sum = 0;

    for q in 1..1001 {
        sum += number_letter_count(q) as i32;
    }

    sum
}
