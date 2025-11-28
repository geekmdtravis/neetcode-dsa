#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;
        let mut head_ptr = head.clone();

        while let Some(mut current) = head_ptr {
            let next = current.next;
            current.next = new_head;
            new_head = Some(current);
            head_ptr = next;
        }
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n3 = Some(Box::new(ListNode::new(1)));
        let n2 = Some(Box::new(ListNode {
            val: 2,
            next: n3.clone(),
        }));
        let n1 = Some(Box::new(ListNode {
            val: 3,
            next: n2.clone(),
        }));

        let reversed = Solution::reverse_list(n3.clone()).unwrap();

        assert_eq!(reversed.val, n3.unwrap().val);
        assert_eq!(reversed.next.clone().unwrap().val, n2.unwrap().val);
        assert_eq!(reversed.next.unwrap().next.unwrap().val, n1.unwrap().val);
    }
}
