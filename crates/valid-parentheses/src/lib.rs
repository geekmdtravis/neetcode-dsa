use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let open_map = HashSet::from(['{', '(', '[']);
        let closed_compliments = HashMap::from([('}', '{'), (')', '('), (']', '[')]);

        let mut opens_queue = Vec::new();
        let mut pairs = Vec::new();

        for c in s.chars() {
            if let Some(_) = open_map.get(&c) {
                opens_queue.push(c);
                continue;
            }
            if let Some(&compliment) = closed_compliments.get(&c) {
                if let Some(&last) = opens_queue.last() {
                    if compliment == last {
                        pairs.push(opens_queue.pop().unwrap());
                        pairs.push(c);
                        continue;
                    }
                }
            }
            return false;
        }

        pairs.len() == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("{}".into(), true; "brackets")]
    #[test_case("{}()".into(), true; "brackets and parenthesis in series")]
    #[test_case("{()}".into(), true; "parenthesis nested inside of brackets")]
    #[test_case("{(})".into(), false; "disordered combination of brackets and parenthesis")]
    #[test_case("{(}".into(), false; "unmatched parenthesis nested in brackets")]
    #[test_case("{".into(), false; "single bracket")]
    #[test_case("[".into(), false; "single square bracket")]
    #[test_case("(".into(), false; "single parenthesis")]
    #[test_case("}".into(), false; "single bracket backward")]
    #[test_case("]".into(), false; "single square bracket backward")]
    #[test_case(")".into(), false; "single parenthesis backward")]
    #[test_case("[]".into(), true; "square brackets")]
    #[test_case("[({})]".into(), true; "square, parenthesis, and brackets nested properly")]
    #[test_case("[){{)[".into(), false; "square, parenthesis, and brackets nested improperly")]
    #[test_case("(([]){})".into(), true; "assymetrical nested of all")]
    #[test_case("((".into(), false; "lacks closing")]

    fn it_works(s: String, expected: bool) {
        let actual = Solution::is_valid(s);
        assert_eq!(actual, expected);
    }
}
