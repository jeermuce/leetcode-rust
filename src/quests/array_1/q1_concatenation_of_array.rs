pub struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        // let mut ans = Vec::with_capacity(nums.len()*2);
        // ans.extend_from_slice(&nums);
        // ans.extend_from_slice(&nums);
        // ans
        nums.repeat(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 1]),
            vec![1, 2, 1, 1, 2, 1]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(
            Solution::get_concatenation(vec![7]),
            vec![7, 7]
        );
    }

    #[test]
    fn all_same() {
        assert_eq!(
            Solution::get_concatenation(vec![5, 5, 5]),
            vec![5, 5, 5, 5, 5, 5]
        );
    }

    #[test]
    fn max_value_examples() {
        assert_eq!(
            Solution::get_concatenation(vec![1000, 1000]),
            vec![1000, 1000, 1000, 1000]
        );
    }
}
