use std::cmp::Ordering;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut high = *piles.iter().max().unwrap();
        let mut low = 1;

        while high > low {
            let mid = low + (high - low) / 2;
            let hours_needed: i32 = piles.iter().map(|pile| (pile + mid - 1) / mid).sum();

            match hours_needed.cmp(&h) {
                Ordering::Greater => {
                    low = mid + 1;
                }
                _ => {
                    high = mid;
                }
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[3,6,7,11], 8, 4)]
    #[test_case(&[30,11,23,4,20], 5, 30)]
    #[test_case(&[30,11,23,4,20], 6, 23)]
    fn finds_min_k(piles: &[i32], hours: i32, expected: i32) {
        let actual = Solution::min_eating_speed(Vec::from(piles), hours);
        assert_eq!(expected, actual);
    }
}
