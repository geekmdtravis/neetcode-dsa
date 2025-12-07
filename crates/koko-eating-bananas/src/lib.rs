use std::cmp::Ordering;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(dead_code)]
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let &max = piles.iter().max().unwrap();
        let k_candidates: Vec<i32> = (1..=max).collect();

        let result = k_candidates.binary_search_by(|&k| {
            let mut total_hours = 0i32;
            for &p in &piles {
                total_hours += (p as f64 / k as f64).ceil() as i32;
            }

            if total_hours > h {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        match result {
            Err(idx) => k_candidates[idx],
            Ok(_) => unreachable!(), // We never return Equal
        }
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

