

#[macro_export]
macro_rules! time {
    ( $x:expr ) => {
        {
            use std::time::{Instant};

            let start = Instant::now();
            let result = { $x };
            let duration = start.elapsed();

            println!("Time elapsed (for {}): {:?}", stringify!($x), duration);

            result
        }
    }
}
