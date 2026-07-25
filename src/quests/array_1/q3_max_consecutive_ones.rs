/* Q3. Max Consecutive Ones
 *
 * Given a binary array `nums`, return the maximum number of consecutive `1`s
 * in the array.
 *
 * # Example 1
 *
 * **Input:**
 * ```text
 * nums = [1,1,0,1,1,1]
 * ```
 *
 * **Output:**
 * ```text
 * 3
 * ```
 *
 * **Explanation:**
 * - The first two digits are consecutive `1`s.
 * - The last three digits are consecutive `1`s.
 * - The maximum number of consecutive `1`s is `3`.
 *
 * # Example 2
 *
 * **Input:**
 * ```text
 * nums = [1,0,1,1,0,1]
 * ```
 *
 * **Output:**
 * ```text
 * 2
 * ```
 *
 * # Constraints
 *
 * - `1 <= nums.length <= 10^5`
 * - `nums[i]` is either `0` or `1`
 */

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
    use rstest::rstest;

    use super::*;

    macro_rules! implementation_tests {
        ($module:ident, $func:path) => {
            mod $module {
                use super::*;

                #[rstest]
                #[case::example_1(vec![1, 1, 0, 1, 1, 1], 3)]
                #[case::example_2(vec![1, 0, 1, 1, 0, 1], 2)]
                #[case::all_ones(vec![1, 1, 1, 1, 1], 5)]
                #[case::all_zeroes(vec![0, 0, 0, 0], 0)]
                #[case::single_one(vec![1], 1)]
                #[case::single_zero(vec![0], 0)]
                #[case::ones_at_start(vec![1, 1, 1, 0, 0, 1], 3)]
                #[case::ones_at_end(vec![0, 0, 1, 1, 1, 1], 4)]
                #[case::alternating_values(vec![1, 0, 1, 0, 1, 0], 1)]
                #[case::multiple_groups_same_length(
                                        vec![1, 1, 0, 1, 1, 0, 1, 1],
                                        2
                                    )]
                fn cases(#[case] nums: Vec<i32>, #[case] expected: i32) {
                    assert_eq!(
                        $func(nums),
                        expected
                    );
                }

                #[test]
                fn large_input() {
                    let nums = vec![1; 100_000];
                    assert_eq!(
                        $func(nums),
                        100_000
                    );
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
