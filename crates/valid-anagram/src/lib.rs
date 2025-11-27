use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars: HashMap<char, i32> = HashMap::with_capacity(s.len());
        let mut t_chars: HashMap<char, i32> = HashMap::with_capacity(t.len());

        for c in s.chars() {
            if let Some(found) = s_chars.get_mut(&c) {
                *found = *found + 1;
            } else {
                s_chars.insert(c, 1);
            }
        }

        for c in t.chars() {
            if let Some(found) = t_chars.get_mut(&c) {
                *found = *found + 1;
            } else {
                t_chars.insert(c, 1);
            }
        }

        s_chars.eq(&t_chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("anagram".into(), "nagaram".into(), true)]
    #[test_case("tab".into(), "bat".into(), true)]
    #[test_case("top".into(), "pot".into(), true)]
    #[test_case("rat".into(), "car".into(), false)]
    fn it_works(s: String, t: String, expected: bool) {
        let actual = Solution::is_anagram(s, t);
        assert_eq!(actual, expected);
    }
}
