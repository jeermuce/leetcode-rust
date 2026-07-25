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
    use crate::test_impls;

    test_impls! {
        types {
            Vec<i32> => i32
        }

        cases {
            example_1            : (vec![1, 1, 0, 1, 1, 1], 3),
            example_2            : (vec![1, 0, 1, 1, 0, 1], 2),
            all_ones             : (vec![1, 1, 1, 1, 1], 5),
            all_zeroes           : (vec![0, 0, 0, 0], 0),
            single_one           : (vec![1], 1),
            single_zero          : (vec![0], 0),
            ones_at_start        : (vec![1, 1, 1, 0, 0, 1], 3),
            ones_at_end          : (vec![0, 0, 1, 1, 1, 1], 4),
            alternating_values   : (vec![1, 0, 1, 0, 1, 0], 1),
            multiple_groups_same_length
                                : (vec![1, 1, 0, 1, 1, 0, 1, 1], 2)
        }

        impls {
            find_max_consecutive_ones_match_flow
                => Solution::find_max_consecutive_ones_match_flow,
            find_max_consecutive_ones_if_flow
                => Solution::find_max_consecutive_ones_if_flow
        }
    }
}
