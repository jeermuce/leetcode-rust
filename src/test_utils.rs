#[macro_export]
macro_rules! unary_cases {
    (
        $module:ident,
        $func:path,

        $(
            $case_name:ident : ($input:expr, $expected:expr)
        ),+ $(,)?
    ) => {
        mod $module {
            use super::*;

            #[rstest::rstest]
            $(
                #[case::$case_name($input, $expected)]
            )+
            fn cases(
                #[case] input: Vec<i32>,
                #[case] expected: i32,
            ) {
                assert_eq!($func(input), expected);
            }
        }
    };
}

#[macro_export]
macro_rules! test_impls {
    (
        cases {
            $(
                $case_name:ident : ($input:expr, $expected:expr)
            ),+ $(,)?
        }

        impls {
            $(
                $impl_name:ident => $func:path
            ),+ $(,)?
        }
    ) => {
        $(
            unary_cases!(
                $impl_name,
                $func,

                $(
                    $case_name : ($input, $expected)
                ),+
            );
        )+
    };
}
