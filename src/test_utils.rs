#[macro_export]
macro_rules! test_solution_variants {
    (
        fn $params:tt -> $out_ty:ty;

        cases $cases:tt

        impls {
            $( $impl_name:ident => $func:path ),+ $(,)?
        }
    ) => {
        $(
            $crate::test_solution_variant_case! {
                fn $params -> $out_ty;
                cases $cases
                impl $impl_name => $func
            }
        )+
    };
}

#[macro_export]
macro_rules! test_solution_variant_case {
    (
        fn ($($pname:ident : $pty:ty),+ $(,)?) -> $out_ty:ty;
        cases {
            $( $case_name:ident : ( $($input:expr),+ $(,)? ) => $expected:expr ),+ $(,)?
        }
        impl $impl_name:ident => $func:path
    ) => {
        mod $impl_name {
            use super::*;

            #[rstest::rstest]
            $(
                #[case::$case_name($($input),+, $expected)]
            )+
            fn cases(
                $(#[case] $pname: $pty,)+
                #[case] expected: $out_ty,
            ) {
                assert_eq!($func($($pname),+), expected);
            }
        }
    };
}
