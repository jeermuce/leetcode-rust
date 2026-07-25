pub struct Solution;

enum Binarium {
    One,
    Zero,
}

impl From<&i32> for Binarium {
    fn from(num: &i32) -> Self {
        if *num == 0 {
            Binarium::Zero
        } else {
            Binarium::One
        }
    }
}

impl Solution {
    pub fn find_max_consecutive_ones_match_enum(nums: Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut max_count = 0;

        for num in nums
            .iter()
            .map(Binarium::from)
        {
            match num {
                Binarium::Zero => curr = 0,
                Binarium::One => {
                    curr += 1;
                    max_count = max_count.max(curr);
                }
            }
        }
        max_count
    }

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
    use crate::test_solution_variants;

    test_solution_variants! {
        fn(nums: Vec<i32>) -> i32;

        cases {
            example_1  : (vec![1, 1, 0, 1, 1, 1]) => 3,
            example_2  : (vec![1, 0, 1, 1, 0, 1]) => 2,
            all_ones   : (vec![1, 1, 1, 1, 1]) => 5,
            all_zeroes : (vec![0, 0, 0, 0]) => 0
        }

        impls {
            find_max_consecutive_ones_match_flow => Solution::find_max_consecutive_ones_match_flow,
            find_max_consecutive_ones_if_flow    => Solution::find_max_consecutive_ones_if_flow,
            find_max_consecutive_ones_match_enum    => Solution::find_max_consecutive_ones_match_enum
        }
    }
}
