#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for row in matrix {
            // Implemented binary search separately and didn't want to duplicate work.
            if row.binary_search(&target).is_ok() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];
        let matrix = vec![a, b, c];
        assert_eq!(Solution::search_matrix(matrix, 5), true);
    }

    #[test]
    fn dnf_appropriate() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];
        let matrix = vec![a, b, c];
        assert_eq!(Solution::search_matrix(matrix, 50), false);
    }
}
