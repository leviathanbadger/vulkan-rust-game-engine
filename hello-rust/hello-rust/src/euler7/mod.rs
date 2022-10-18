use crate::util::{prime_sieve};

fn nth_prime(n: usize) -> i32 {
    let mut check_up_to = 25000;
    let mut primes = vec![0; 0];

    while primes.len() < (n + 1) {
        primes = prime_sieve(check_up_to);
        check_up_to *= 2;
    }

    primes[n]
}

#[allow(dead_code)]
pub fn euler7() {
    let answer = nth_prime(10001 - 1); //0-indexed, so have to subtract 1
    println!("Euler 7: {}", answer);
}
