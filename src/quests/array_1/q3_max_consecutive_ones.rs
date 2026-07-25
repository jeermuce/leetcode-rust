pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
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
    // pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    //     let mut curr = 0;
    //     let mut max_count = 0;
    //     for num in nums {
    //         if num == 0 {
    //             curr = 0;
    //             continue;
    //         }
    //
    //         curr += 1;
    //         max_count = max_count.max(curr);
    //     }
    //
    //     max_count
    // }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![
            1, 1, 0, 1, 1, 1,
        ];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            3
        );
    }

    #[test]
    fn test_example_2() {
        let nums = vec![
            1, 0, 1, 1, 0, 1,
        ];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            2
        );
    }

    #[test]
    fn test_all_ones() {
        let nums = vec![
            1, 1, 1, 1, 1,
        ];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            5
        );
    }

    #[test]
    fn test_all_zeroes() {
        let nums = vec![
            0, 0, 0, 0,
        ];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            0
        );
    }

    #[test]
    fn test_single_one() {
        let nums = vec![1];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            1
        );
    }

    #[test]
    fn test_single_zero() {
        let nums = vec![0];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            0
        );
    }

    #[test]
    fn test_ones_at_start() {
        let nums = vec![
            1, 1, 1, 0, 0, 1,
        ];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            3
        );
    }

    #[test]
    fn test_ones_at_end() {
        let nums = vec![
            0, 0, 1, 1, 1, 1,
        ];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            4
        );
    }

    #[test]
    fn test_alternating_values() {
        let nums = vec![
            1, 0, 1, 0, 1, 0,
        ];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            1
        );
    }

    #[test]
    fn test_multiple_groups_same_length() {
        let nums = vec![
            1, 1, 0, 1, 1, 0, 1, 1,
        ];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            2
        );
    }

    #[test]
    fn test_large_input() {
        let nums = vec![1; 100000];
        assert_eq!(
            Solution::find_max_consecutive_ones(nums),
            100000
        );
    }
}
