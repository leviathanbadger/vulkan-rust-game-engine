

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
    };
    ( $x:expr , $($arg:tt)* ) => {
        {
            let action = format!( $($arg)*, );

            use std::time::{Instant};

            println!();
            println!("{}...", action);

            let start = Instant::now();
            let result = { $x };
            let duration = start.elapsed();

            println!("{} took {:?}; result: {:?}", action, duration, result);

            result
        }
    }
}
