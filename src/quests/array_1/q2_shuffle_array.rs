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
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![
            2, 5, 1, 3, 4, 7,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            2, 3, 5, 4, 1, 7,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn example_2() {
        let nums = vec![
            1, 2, 3, 4, 4, 3, 2, 1,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            1, 4, 2, 3, 3, 2, 4, 1,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn example_3() {
        let nums = vec![
            1, 1, 2, 2,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            1, 2, 1, 2,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn minimum_size() {
        let nums = vec![
            1, 1000,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            1, 1000,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn all_same_values() {
        let nums = vec![
            7, 7, 7, 7, 7, 7,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            7, 7, 7, 7, 7, 7,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn alternating_after_shuffle() {
        let nums = vec![
            10, 20, 30, 40, 50, 60,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            10, 40, 20, 50, 30, 60,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn increasing_sequence() {
        let nums = vec![
            1, 2, 3, 4, 5, 6, 7, 8,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            1, 5, 2, 6, 3, 7, 4, 8,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn repeated_pairs() {
        let nums = vec![
            1, 2, 1, 2, 3, 4, 3, 4,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            1, 3, 2, 4, 1, 3, 2, 4,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn boundary_values() {
        let nums = vec![
            1, 1000, 1, 1000,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            1, 1, 1000, 1000,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn larger_case() {
        let nums = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        let n = (nums.len() / 2) as i32;
        let expected = vec![
            1, 6, 2, 7, 3, 8, 4, 9, 5, 10,
        ];

        assert_eq!(
            Solution::shuffle(nums, n),
            expected
        );
    }

    #[test]
    fn maximum_n_pattern() {
        let mut nums = Vec::with_capacity(1000);

        nums.extend(1..=500);
        nums.extend(501..=1000);

        let n = (nums.len() / 2) as i32;

        let result = Solution::shuffle(
            nums, n,
        );

        assert_eq!(
            result.len(),
            1000
        );

        for i in 0..500 {
            assert_eq!(
                result[2 * i],
                (i + 1) as i32
            );
            assert_eq!(
                result[2 * i + 1],
                (i + 501) as i32
            );
        }
    }
}
