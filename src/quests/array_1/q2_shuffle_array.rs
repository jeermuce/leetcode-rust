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
    use crate::test_solution_variants;

    test_solution_variants! {
        fn(nums: Vec<i32>, n: i32) -> Vec<i32>;

        cases {
            example_1 : (vec![2, 5, 1, 3, 4, 7], 3) => vec![2, 3, 5, 4, 1, 7],
            example_2 : (vec![1, 2, 3, 4, 4, 3, 2, 1], 4) => vec![1, 4, 2, 3, 3, 2, 4, 1],
            all_pairs : (vec![1, 1, 2, 2], 2) => vec![1, 2, 1, 2]
        }

        impls {
            shuffle => Solution::shuffle
        }
    }
}
