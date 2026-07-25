/* Q2. Shuffle the Array
 *
 * Given the array `nums` consisting of `2n` elements in the form:
 *
 * ```text
 * [x1, x2, ..., xn, y1, y2, ..., yn]
 * ```
 *
 * Return the array in the form:
 *
 * ```text
 * [x1, y1, x2, y2, ..., xn, yn]
 * ```
 *
 * # Example 1
 *
 * **Input:**
 * ```text
 * nums = [2,5,1,3,4,7], n = 3
 * ```
 *
 * **Output:**
 * ```text
 * [2,3,5,4,1,7]
 * ```
 *
 * **Explanation:**
 * - `x1 = 2`
 * - `x2 = 5`
 * - `x3 = 1`
 * - `y1 = 3`
 * - `y2 = 4`
 * - `y3 = 7`
 *
 * Therefore:
 *
 * ```text
 * [x1, y1, x2, y2, x3, y3] = [2,3,5,4,1,7]
 * ```
 *
 * # Example 2
 *
 * **Input:**
 * ```text
 * nums = [1,2,3,4,4,3,2,1], n = 4
 * ```
 *
 * **Output:**
 * ```text
 * [1,4,2,3,3,2,4,1]
 * ```
 *
 * # Example 3
 *
 * **Input:**
 * ```text
 * nums = [1,1,2,2], n = 2
 * ```
 *
 * **Output:**
 * ```text
 * [1,2,1,2]
 * ```
 *
 * # Constraints
 *
 * - `1 <= n <= 500`
 * - `nums.length == 2n`
 * - `1 <= nums[i] <= 10^3`
 */
pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let (x, y) = nums.split_at(n as usize);
        x.iter()
            .zip(y)
            .flat_map(
                |(&a, &b)| {
                    [
                        a, b,
                    ]
                },
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![2, 5, 1, 3, 4, 7], 3, vec![2, 3, 5, 4, 1, 7])]
    #[case(vec![1, 2, 3, 4, 4, 3, 2, 1], 4, vec![1, 4, 2, 3, 3, 2, 4, 1])]
    #[case(vec![1, 1, 2, 2], 2, vec![1, 2, 1, 2])]
    #[case(vec![1, 1000], 1, vec![1, 1000])]
    #[case(vec![7, 7, 7, 7, 7, 7], 3, vec![7, 7, 7, 7, 7, 7])]
    #[case(vec![10, 20, 30, 40, 50, 60], 3, vec![10, 40, 20, 50, 30, 60])]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8], 4, vec![1, 5, 2, 6, 3, 7, 4, 8])]
    #[case(vec![1, 2, 1, 2, 3, 4, 3, 4], 4, vec![1, 3, 2, 4, 1, 3, 2, 4])]
    #[case(vec![1, 1000, 1, 1000], 2, vec![1, 1, 1000, 1000])]
    #[case(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        5,
        vec![1, 6, 2, 7, 3, 8, 4, 9, 5, 10]
    )]
    fn cases(
        #[case] nums: Vec<i32>,
        #[case] n: i32,
        #[case] expected: Vec<i32>,
    ) {
        assert_eq!(Solution::shuffle(nums, n), expected);
    }

    #[test]
    fn maximum_n_pattern() {
        let mut nums = Vec::with_capacity(1000);
        nums.extend(1..=500);
        nums.extend(501..=1000);

        let result = Solution::shuffle(nums, 500);

        assert_eq!(result.len(), 1000);

        for i in 0..500 {
            assert_eq!(result[2 * i], (i + 1) as i32);
            assert_eq!(result[2 * i + 1], (i + 501) as i32);
        }
    }
}
