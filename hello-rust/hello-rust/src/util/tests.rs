#![cfg(test)]

#[macro_export]
macro_rules! test_cases {
    ( $name:ident , [ $( ( $case_name:ident , $input:expr , $expected:expr ) ),* ] , ( $input_ident:ident , $expected_ident:ident ) $fn:block ) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;
        $(
            #[test]
            fn $case_name() {
                let $input_ident = $input;
                let $expected_ident = $expected;

                $fn
            }
        )*
        }
    };

    ( $name:ident , [ $( ( $case_name:ident , $expected:expr ) ),* ] , ( $expected_ident:ident ) $fn:block ) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;
        $(
            #[test]
            fn $case_name() {
                let $expected_ident = $expected;

                $fn
            }
        )*
        }
    }
}
