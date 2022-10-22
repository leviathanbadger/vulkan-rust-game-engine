mod tests;

use std::{collections::HashSet, sync::Mutex, sync::Arc, thread};

use crate::util::{prime_sieve};

fn consecutive_quadratic_primes(a: i32, b: i32, primes: &HashSet<i32>) -> i32 {
    let mut n = 0 as i32;

    loop {
        let val = ((n + a) * n) + b;
        if primes.contains(&val) {
            n += 1;
        }
        else {
            break;
        }
    }

    n
}

struct Euler27State {
    unchecked_a: Vec<i32>,
    best_a: i32,
    best_b: i32,
    max_n: i32
}

fn euler27_thread(state_mut: Arc<Mutex<Euler27State>>, primes: &HashSet<i32>) {
    let mut best_a: i32 = 0;
    let mut best_b: i32 = 0;
    let mut max_n: i32 = -1;

    loop {
        let a: i32;
        {
            let mut state = state_mut.lock().unwrap();

            if max_n > state.max_n {
                state.best_a = best_a;
                state.best_b = best_b;
                state.max_n = max_n;
            }

            if let Some(unwrap_a) = state.unchecked_a.pop() {
                a = unwrap_a;
            }
            else {
                break;
            }
        }

        for b in -1000..=1000 {
            let n = consecutive_quadratic_primes(a, b, &primes);
            if n > max_n {
                max_n = n;
                best_a = a;
                best_b = b;
            }
        }
    }
}

#[allow(dead_code)]
pub fn euler27() -> i32 {
    let primes_vec = prime_sieve(1_000_000);
    let primes = HashSet::from_iter(primes_vec);

    let state_mut = Arc::new(Mutex::new(Euler27State {
        unchecked_a: Vec::from_iter(-999..=999),
        best_a: 0,
        best_b: 0,
        max_n: 0
    }));

    const NUM_THREADS: i32 = 16;
    let mut handles = vec![];
    for _q in 0..NUM_THREADS {
        let state_mut_cp = Arc::clone(&state_mut);
        let primes_cp = primes.clone();
        let handle = thread::spawn(move || {
            euler27_thread(state_mut_cp, &primes_cp);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let state = state_mut.lock().unwrap();
    state.best_a * state.best_b
}
