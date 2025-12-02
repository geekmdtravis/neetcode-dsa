#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        Solution::climb_stairs_imperative(n)
    }

    pub fn climb_stairs_imperative(n: i32) -> i32 {
        let mut step_a = 1; // Ways to get to ground (0th step).
        let mut step_b = 1; // Ways to get to first step (1st step).

        for _i in 2..=n {
            let step_c = step_a + step_b; // Wayst to get to ith step
            step_a = step_b; // Advance
            step_b = step_c; // Advance 
        }

        step_b
    }

    #[allow(dead_code)]
    pub fn climb_stairs_declarative(n: i32) -> i32 {
        (0..n).fold((1, 1), |(a, b), _| (b, a + b)).0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case(1, 1)]
    #[test_case(2, 2)]
    #[test_case(3, 3)]
    #[test_case(4, 5)]
    #[test_case(5, 8)]
    fn test_example_1(n: i32, expected: i32) {
        assert_eq!(Solution::climb_stairs(n), expected);
    }
}
