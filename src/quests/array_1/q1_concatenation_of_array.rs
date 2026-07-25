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
    use super::*;
    use crate::test_impls;

    test_impls! {
        fn(nums: Vec<i32>) -> Vec<i32>;

        cases {
            example_1 : (vec![1, 2, 1]) => vec![1, 2, 1, 1, 2, 1],
            example_2 : (vec![1, 3, 2, 1]) => vec![1, 3, 2, 1, 1, 3, 2, 1],
            single    : (vec![7]) => vec![7, 7],
            repeated  : (vec![5, 5, 5]) => vec![5, 5, 5, 5, 5, 5],
            max_vals  : (vec![1000, 1000]) => vec![1000, 1000, 1000, 1000]
        }

        impls {
            get_concatenation_repeat => Solution::get_concatenation_repeat,
            get_concatenation_extend => Solution::get_concatenation_extend
        }
    }
}
