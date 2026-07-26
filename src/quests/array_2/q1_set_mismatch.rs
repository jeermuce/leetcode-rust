use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let good_sum = (nums.len() * (nums.len() + 1) / 2) as i32;
        let bad_sum: i32 = nums
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .sum();
        let ugly_sum = nums
            .iter()
            .sum::<i32>();

        vec![
            (ugly_sum - bad_sum),
            (good_sum - bad_sum),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_solution_variants;

    test_solution_variants! {
            fn(nums: Vec<i32>) -> Vec<i32>;

    cases {
        example_1 : (vec![1, 2, 2, 4]) => vec![2, 3],
        example_2 : (vec![1, 1]) => vec![1, 2],

        duplicate_smallest_missing_middle : (vec![1, 1, 3, 4, 5]) => vec![1, 2],
        duplicate_smallest_missing_largest : (vec![1, 1, 2, 3]) => vec![1, 4],
        duplicate_largest_missing_smallest : (vec![2, 3, 4, 4]) => vec![4, 1],
        duplicate_largest_missing_middle : (vec![1, 2, 4, 4]) => vec![4, 3],

        duplicate_middle_missing_middle_low : (vec![1, 2, 2, 4, 5]) => vec![2, 3],
        duplicate_middle_missing_middle_high : (vec![1, 2, 3, 3, 5]) => vec![3, 4],

        duplicate_first_in_larger_set : (vec![1, 1, 3, 4, 5, 6, 7]) => vec![1, 2],
        duplicate_last_in_larger_set : (vec![1, 2, 3, 4, 5, 6, 6]) => vec![6, 7],
        duplicate_middle_in_larger_set : (vec![1, 2, 3, 5, 5, 6, 7]) => vec![5, 4],

        duplicate_near_start : (vec![1, 2, 2, 3, 4, 5]) => vec![2, 6],
        duplicate_near_end : (vec![1, 2, 3, 4, 5, 5]) => vec![5, 6],

        only_two_numbers_duplicate_first : (vec![1, 1]) => vec![1, 2],
        only_two_numbers_duplicate_second : (vec![2, 2]) => vec![2, 1],

        larger_gap_before_missing : (vec![1, 2, 3, 3, 5, 6, 7, 8]) => vec![3, 4],
        larger_gap_after_missing : (vec![1, 2, 3, 4, 6, 6, 7, 8]) => vec![6, 5],
        duplicate_high_value : (vec![1, 2, 3, 4, 5, 8, 8, 7]) => vec![8, 6],

        duplicate_low_value_large_range : (vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 9]) => vec![1, 10],
        duplicate_high_value_large_range : (vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 10]) => vec![10, 9],
    }

            impls {
                find_error_nums => Solution::find_error_nums,
            }
        }
}
