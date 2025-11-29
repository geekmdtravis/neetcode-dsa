// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy.next;

        while let (Some(l1), Some(l2)) = (&list1, &list2) {
            if l1.val <= l2.val {
                *tail = list1.take();
                tail = &mut tail.as_mut().unwrap().next;
                list1 = tail.take();
            } else {
                *tail = list2.take();
                tail = &mut tail.as_mut().unwrap().next;
                list2 = tail.take();
            }
        }
        *tail = list1.or(list2);
        dummy.next
    }
}
