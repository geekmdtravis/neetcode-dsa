#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut idx_low = 0;
        let mut idx_high = nums.len() - 1;

        while idx_high > idx_low {
            let idx_mid = (idx_high + idx_low) / 2;

            if target < nums[idx_mid] {
                idx_high = if idx_mid > 0 { idx_mid - 1 } else { idx_mid };
            } else if target > nums[idx_mid] {
                idx_low = idx_mid + 1;
            } else {
                return idx_mid as i32;
            }
        }

        if nums[idx_high] == target {
            return idx_high as i32;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[5], 5, 0; "simple pos")]
    #[test_case(&[5], -5, -1; "simple neg")]
    #[test_case(&[2,5], 0, -1)]
    #[test_case(&[2,5], 2, 0)]
    #[test_case(&[-1,0,3,5,9,12], 9, 4)]
    #[test_case(&[-1,0,3,5,9,12], 2, -1)]
    fn it_works(arr: &[i32], target: i32, expected_idx: i32) {
        let actual_idx = Solution::search(Vec::from(arr), target);
        assert_eq!(actual_idx, expected_idx);
    }
}
