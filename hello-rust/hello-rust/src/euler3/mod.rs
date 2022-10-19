mod tests;

use crate::util::{prime_sieve};

fn largest_prime_factor(of_num: i64) -> i32 {
    let max_check = f32::floor(f32::sqrt(of_num as f32)) as usize;
    let primes = prime_sieve(max_check);

    // println!("{:?}", primes);

    let mut num = of_num;
    for prime in primes {
        let prime64 = prime as i64;
        while num % prime64 == 0 {
            // println!("{} is divisible by {}", num, prime64);
            num /= prime64;
            if num == 1 {
                return prime
            }
        }
    }

    panic!("Could not find the largest prime factor. This should theoretically be impossible.");
}

#[allow(dead_code)]
pub fn euler3() -> i32 {
    let result = largest_prime_factor(600851475143i64);

    result
}
