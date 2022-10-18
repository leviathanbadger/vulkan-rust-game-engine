

// TODO: allow results to grow if the resultant prime count is not enough

pub fn prime_sieve(max: usize) -> Vec<i32> {
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
