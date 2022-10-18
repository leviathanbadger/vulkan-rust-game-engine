

fn is_palindromic(slice: &[i32]) -> bool {
    let max = slice.len() / 2;
    let mut is_palindromic = true;

    for q in 0..(max + 1) {
        if slice[q] != slice[slice.len() - 1 - q] {
            is_palindromic = false;
            break;
        }
    }

    is_palindromic
}

fn largest_palindrome_product(min: i32, max: i32) -> i32 {
    let mut buff = [0 as i32; 10];
    let mut max_product = 0;

    for q in min..(max + 1) {
        for w in min..(q + 1) {
            let product = q * w;
            if product < max_product { continue; }

            let mut e = product;
            let mut idx = 0;
            while e != 0 {
                buff[idx] = e % 10;
                e /= 10;
                idx += 1;
            }

            if is_palindromic(&buff[0..idx]) {
                max_product = product;
            }
        }
    }

    max_product
}

#[allow(dead_code)]
pub fn euler4() {
    let answer = largest_palindrome_product(100, 999);
    println!("Euler 4: {}", answer);
}
