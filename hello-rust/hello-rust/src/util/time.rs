

#[macro_export]
macro_rules! time {
    ( $x:expr ) => {
        {
            use std::time::{Instant};

            println!();
            println!("Executing {:?}...", stringify!($x));

            let start = Instant::now();
            let result = { $x };
            let duration = start.elapsed();

            println!("{}; result: {:?}; time elapsed: {:?}", stringify!($x), result, duration);

            result
        }
    }
}
