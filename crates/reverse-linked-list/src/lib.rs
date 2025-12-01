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
