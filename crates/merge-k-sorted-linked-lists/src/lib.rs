#[allow(dead_code)]
struct Solution;

/// Definition for singly-linked list.
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

    #[allow(dead_code)]
    fn values(self) -> Vec<i32> {
        let mut values = vec![];
        let mut curr = &self;

        while let Some(next) = &curr.next {
            values.push(curr.val);
            curr = next.as_ref();
        }
        values
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let taken = lists.get_mut(0).take().unwrap().take();
        taken
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_and_sorts_three_lists() {
        let mut node_a1 = ListNode::new(11);
        let mut node_a2 = ListNode::new(20);
        let node_a3 = ListNode::new(33);
        node_a2.next = Some(Box::new(node_a3));
        node_a1.next = Some(Box::new(node_a2));

        let mut node_b1 = ListNode::new(0);
        let mut node_b2 = ListNode::new(2);
        let mut node_b3 = ListNode::new(10);
        let node_b4 = ListNode::new(30);
        node_b3.next = Some(Box::new(node_b4));
        node_b2.next = Some(Box::new(node_b3));
        node_b1.next = Some(Box::new(node_b2));

        let mut node_c1 = ListNode::new(3);
        let mut node_c2 = ListNode::new(42);
        let mut node_c3 = ListNode::new(105);
        let mut node_c4 = ListNode::new(136);
        let node_c5 = ListNode::new(1009);
        node_c4.next = Some(Box::new(node_c5));
        node_c3.next = Some(Box::new(node_c4));
        node_c2.next = Some(Box::new(node_c3));
        node_c1.next = Some(Box::new(node_c2));

        let list = vec![
            Some(Box::new(node_a1)),
            Some(Box::new(node_b1)),
            Some(Box::new(node_c1)),
        ];

        println!("{:?}", list.clone());

        let head = Solution::merge_k_lists(list);
        let actual_order = head.unwrap().values();
        println!("{:?}", actual_order.clone());

        let mut expected_order = vec![11, 20, 33, 0, 2, 10, 3, 42, 105, 1009];
        expected_order.sort();
        println!("{:?}", expected_order.clone());

        assert_eq!(actual_order.len(), expected_order.len());
        assert_eq!(actual_order, expected_order);
    }
}
