#[macro_export]
macro_rules! test_impls {
    (
        fn $params:tt -> $out_ty:ty;

        cases $cases:tt

        impls {
            $( $impl_name:ident => $func:path ),+ $(,)?
        }
    ) => {
        $(
            $crate::test_impls_one! {
                fn $params -> $out_ty;
                cases $cases
                impl $impl_name => $func
            }
        )+
    };
}

#[macro_export]
macro_rules! test_impls_one {
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
