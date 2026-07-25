pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones_match_flow(nums: Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut max_count = 0;
        for num in nums {
            match num {
                0 => curr = 0,
                1 => {
                    curr += 1;
                    max_count = max_count.max(curr)
                }
                _ => unreachable!(),
            }
        }
        max_count
    }

    pub fn find_max_consecutive_ones_if_flow(nums: Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut max_count = 0;
        for num in nums {
            if num == 0 {
                curr = 0;
                continue;
            }

            curr += 1;
            max_count = max_count.max(curr);
        }

        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_example_1<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![
            1, 1, 0, 1, 1, 1,
        ];
        assert_eq!(
            f(nums),
            3
        );
    }

    fn test_example_2<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![
            1, 0, 1, 1, 0, 1,
        ];
        assert_eq!(
            f(nums),
            2
        );
    }

    fn test_all_ones<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![
            1, 1, 1, 1, 1,
        ];
        assert_eq!(
            f(nums),
            5
        );
    }

    fn test_all_zeroes<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![
            0, 0, 0, 0,
        ];
        assert_eq!(
            f(nums),
            0
        );
    }

    fn test_single_one<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![1];
        assert_eq!(
            f(nums),
            1
        );
    }

    fn test_single_zero<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![0];
        assert_eq!(
            f(nums),
            0
        );
    }

    fn test_ones_at_start<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![
            1, 1, 1, 0, 0, 1,
        ];
        assert_eq!(
            f(nums),
            3
        );
    }

    fn test_ones_at_end<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![
            0, 0, 1, 1, 1, 1,
        ];
        assert_eq!(
            f(nums),
            4
        );
    }

    fn test_alternating_values<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![
            1, 0, 1, 0, 1, 0,
        ];
        assert_eq!(
            f(nums),
            1
        );
    }

    fn test_multiple_groups_same_length<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![
            1, 1, 0, 1, 1, 0, 1, 1,
        ];
        assert_eq!(
            f(nums),
            2
        );
    }

    fn test_large_input<F>(f: F)
    where
        F: Fn(Vec<i32>) -> i32,
    {
        let nums = vec![1; 100_000];
        assert_eq!(
            f(nums),
            100_000
        );
    }

    macro_rules! implementation_tests {
        ($module:ident, $func:path) => {
            mod $module {
                use super::*;

                #[test]
                fn example_1() {
                    test_example_1($func);
                }

                #[test]
                fn example_2() {
                    test_example_2($func);
                }

                #[test]
                fn all_ones() {
                    test_all_ones($func);
                }

                #[test]
                fn all_zeroes() {
                    test_all_zeroes($func);
                }

                #[test]
                fn single_one() {
                    test_single_one($func);
                }

                #[test]
                fn single_zero() {
                    test_single_zero($func);
                }

                #[test]
                fn ones_at_start() {
                    test_ones_at_start($func);
                }

                #[test]
                fn ones_at_end() {
                    test_ones_at_end($func);
                }

                #[test]
                fn alternating_values() {
                    test_alternating_values($func);
                }

                #[test]
                fn multiple_groups_same_length() {
                    test_multiple_groups_same_length($func);
                }

                #[test]
                fn large_input() {
                    test_large_input($func);
                }
            }
        };
    }

    implementation_tests!(
        find_max_consecutive_ones_match_flow,
        Solution::find_max_consecutive_ones_match_flow
    );

    implementation_tests!(
        find_max_consecutive_ones_if_flow,
        Solution::find_max_consecutive_ones_if_flow
    );
}
