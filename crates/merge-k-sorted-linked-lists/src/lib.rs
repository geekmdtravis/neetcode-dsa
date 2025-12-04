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
        let mut values = vec![self.val];
        let mut cur = self.next;

        while let Some(node) = cur {
            values.push(node.val);
            cur = node.next;
        }

        values
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut outer_curr = dummy.as_mut();

        for list_head in lists {
            let mut inner_curr = list_head;

            while let Some(mut node) = inner_curr {
                inner_curr = node.next.take();
                outer_curr.next = Some(node);
                outer_curr = outer_curr.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_returns() {
        let mut node_a1 = ListNode::new(11);
        let mut node_a2 = ListNode::new(20);
        let node_a3 = ListNode::new(33);
        node_a2.next = Some(Box::new(node_a3));
        node_a1.next = Some(Box::new(node_a2));

        let actual = node_a1.values();
        let expected = vec![11, 20, 33];

        assert_eq!(actual, expected);
    }

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
        let actual_order = match head {
            Some(h) => h.values(),
            None => vec![],
        };
        println!("{:?}", actual_order.clone());

        let mut expected_order = vec![11, 20, 33, 0, 2, 10, 30, 3, 42, 105, 136, 1009];
        expected_order.sort();
        println!("{:?}", expected_order.clone());

        assert_eq!(actual_order.len(), expected_order.len());
        assert_eq!(actual_order, expected_order);
    }
}
