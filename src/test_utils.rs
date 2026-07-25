#[macro_export]
macro_rules! test_impls {
    (
        types { $input_ty:ty => $output_ty:ty }
        cases $cases:tt
        impls {
            $( $impl_name:ident => $func:path ),+ $(,)?
        }
    ) => {
        $(
            $crate::test_impls!(@impl $impl_name, $func, $input_ty, $output_ty, $cases);
        )+
    };

    (@impl
        $impl_name:ident, $func:path, $input_ty:ty, $output_ty:ty,
        { $( $case_name:ident : ($input:expr, $expected:expr) ),+ $(,)? }
    ) => {
        mod $impl_name {
            use super::*;

            #[rstest::rstest]
            $(
                #[case($input, $expected)]
            )+
            fn cases(
                #[case] input: $input_ty,
                #[case] expected: $output_ty,
            ) {
                assert_eq!($func(input), expected);
            }
        }
    };
}
