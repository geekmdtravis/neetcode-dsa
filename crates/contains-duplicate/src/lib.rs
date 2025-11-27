use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

impl Solution {
    /// Given an integer array nums, return true if any
    /// value appears more than once in the array,
    /// otherwise return false.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers.
    ///
    /// # Returns
    /// * `bool` - True if any value appears more than once, false otherwise.
    ///
    #[allow(dead_code)]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut complement_map = HashSet::with_capacity(nums.len());

        for num in nums {
            if complement_map.contains(&num) {
                return true;
            } else {
                complement_map.insert(num);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,2,3,1], true)]
    #[test_case(vec![1,2,3], false)]
    #[test_case(vec![1,2,3,3], true)]
    #[test_case(vec![1,2,2,0], true)]
    #[test_case(vec![1,1,2,0], true)]
    fn it_works(list: Vec<i32>, expected: bool) {
        let result = Solution::contains_duplicate(list);
        assert_eq!(result, expected);
    }
}
