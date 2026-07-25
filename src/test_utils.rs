#[doc(hidden)]
#[macro_export]
macro_rules! __test_solution_variant_case {
    (
        fn ($($pname:ident : $pty:ty),+ $(,)?) -> $out_ty:ty;
        cases {
            $( $case_name:ident : ( $($input:expr),+ $(,)? ) => $expected:expr ),+ $(,)?
        }
        body |$actual:ident, $expected_ident:ident| $body:block
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
                let actual = $func($($pname),+);

                let $actual = actual;
                let $expected_ident = expected;

                $body
            }
        }
    };
}

#[macro_export]
macro_rules! test_solution_variant_case {
    (
        fn $params:tt -> $out_ty:ty;
        cases $cases:tt
        impl $impl_name:ident => $func:path
    ) => {
        $crate::__test_solution_variant_case! {
            fn $params -> $out_ty;
            cases $cases
            body |actual, expected| {
                assert_eq!(actual, expected);
            }
            impl $impl_name => $func
        }
    };
}

#[macro_export]
macro_rules! test_solution_variant_case_with {
    (
        fn $params:tt -> $out_ty:ty;
        cases $cases:tt
        assert $assertion:expr;
        impl $impl_name:ident => $func:path
    ) => {
        $crate::__test_solution_variant_case! {
            fn $params -> $out_ty;
            cases $cases
            body |actual, expected| {
                ($assertion)(actual, expected);
            }
            impl $impl_name => $func
        }
    };
}

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
macro_rules! test_solution_variants_with {
    (
        fn $params:tt -> $out_ty:ty;

        cases $cases:tt

        assert $assertion:expr;

        impls {
            $( $impl_name:ident => $func:path ),+ $(,)?
        }
    ) => {
        $(
            $crate::test_solution_variant_case_with! {
                fn $params -> $out_ty;
                cases $cases
                assert $assertion;
                impl $impl_name => $func
            }
        )+
    };
}
