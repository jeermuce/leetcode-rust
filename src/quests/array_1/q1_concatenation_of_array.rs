/* Q1. Concatenation of Array
 *
 * Given an integer array `nums` of length `n`, create an array `ans` of length `2n` such that:
 *
 * - `ans[i] == nums[i]`
 * - `ans[i + n] == nums[i]`
 *
 * for `0 <= i < n` (0-indexed).
 *
 * Specifically, `ans` is the concatenation of two `nums` arrays.
 *
 * Return the array `ans`.
 *
 * # Example 1
 *
 * **Input:**
 * ```text
 * nums = [1,2,1]
 * ```
 *
 * **Output:**
 * ```text
 * [1,2,1,1,2,1]
 * ```
 *
 * **Explanation:**
 * - `ans = [nums[0], nums[1], nums[2], nums[0], nums[1], nums[2]]`
 * - `ans = [1,2,1,1,2,1]`
 *
 * # Example 2
 *
 * **Input:**
 * ```text
 * nums = [1,3,2,1]
 * ```
 *
 * **Output:**
 * ```text
 * [1,3,2,1,1,3,2,1]
 * ```
 *
 * **Explanation:**
 * - `ans = [nums[0], nums[1], nums[2], nums[3], nums[0], nums[1], nums[2], nums[3]]`
 * - `ans = [1,3,2,1,1,3,2,1]`
 *
 * # Constraints
 *
 * - `n == nums.length`
 * - `1 <= n <= 1000`
 * - `1 <= nums[i] <= 1000`
 */

pub struct Solution;

impl Solution {
    pub fn get_concatenation_repeat(nums: Vec<i32>) -> Vec<i32> {
        nums.repeat(2)
    }

    pub fn get_concatenation_extend(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len() * 2);
        ans.extend_from_slice(&nums);
        ans.extend_from_slice(&nums);
        ans
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
                #[case(vec![1, 2, 1], vec![1, 2, 1, 1, 2, 1])]
                #[case(vec![1, 3, 2, 1], vec![1, 3, 2, 1, 1, 3, 2, 1])]
                #[case(vec![7], vec![7, 7])]
                #[case(vec![5, 5, 5], vec![5, 5, 5, 5, 5, 5])]
                #[case(vec![1000, 1000], vec![1000, 1000, 1000, 1000])]
                fn cases(
                    #[case] nums: Vec<i32>,
                    #[case] expected: Vec<i32>,
                ) {
                    assert_eq!($func(nums), expected);
                }
            }
        };
    }

    implementation_tests!(
        get_concatenation_repeat,
        Solution::get_concatenation_repeat
    );

    implementation_tests!(
        get_concatenation_extend,
        Solution::get_concatenation_extend
    );
}
