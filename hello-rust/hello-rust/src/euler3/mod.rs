

fn prime_sieve(max: usize) -> Vec<i32> {
    // let data = Vec::<bool>::new();
    // data.resize(max as usize, true);
    let mut data = vec![true; max + 1]; //Pretending to be 1-indexed instead of 0-indexed to keep logic readable
    data[0] = false;
    data[1] = false;

    let mut primes = vec![0 as i32; 0];

    for q in 2..max {
        if data[q] {
            primes.push(q as i32);
            let mut w = q * 2;
            loop {
                if w >= data.len() { break; }
                data[w] = false;
                w += q;
            }
        }
    }

    primes
}

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
pub fn euler3() {
    let answer = largest_prime_factor(600851475143i64);
    println!("Euler 3: {}", answer);
}
