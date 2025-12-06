use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut buckets = HashMap::with_capacity(3);

        for n in nums.iter() {
            buckets.entry(*n).and_modify(|n| *n += 1).or_insert(1);
        }

        println!("{:?}", buckets);

        let mut buckets = buckets.into_iter().collect::<Vec<(i32, i32)>>();
        buckets.sort_unstable_by_key(|b| b.0);

        let mut offset = 0;
        for b in buckets {
            for i in offset..offset + b.1 {
                nums[i as usize] = b.0;
            }
            offset += b.1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[2,0,2,1,1,0], &[0,0,1,1,2,2])]
    #[test_case(&[2,0,1], &[0,1,2])]
    fn it_works(list: &[i32], expected: &[i32]) {
        let expected = Vec::from(expected);

        let mut actual = Vec::from(list);
        Solution::sort_colors(&mut actual);

        assert_eq!(actual, expected)
    }
}
