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

    fn from_values(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut anchor = Box::new(ListNode::new(0));
        let mut builder = &mut anchor;

        for val in values {
            builder = builder.next.insert(Box::new(ListNode::new(val)));
        }

        anchor.next
    }
}

// TODO: Opted out of recursive merge sort implementation for now; will
// revisit later when time permits.
impl Solution {
    #[allow(dead_code)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let (head_node, _) = Solution::concat_lists(lists);

        let mut values = head_node.unwrap().values();
        values.sort();

        ListNode::from_values(values)
    }

    fn concat_lists(lists: Vec<Option<Box<ListNode>>>) -> (Option<Box<ListNode>>, i32) {
        let mut anchor = Box::new(ListNode::new(0));
        let mut builder = &mut anchor;
        let mut node_cnt = 0i32;

        for mut curr in lists {
            while let Some(mut node) = curr {
                curr = node.next.take();
                builder = builder.next.insert(node);
                node_cnt += 1;
            }
        }

        (anchor.next, node_cnt)
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
    fn concat_list() {
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

        let (head, cnt) = Solution::concat_lists(list);
        let actual_order = match head {
            Some(h) => h.values(),
            None => vec![],
        };

        let expected_order = vec![11, 20, 33, 0, 2, 10, 30, 3, 42, 105, 136, 1009];

        assert_eq!(cnt, expected_order.len() as i32);
        assert_eq!(actual_order, expected_order);
    }

    #[test]
    fn sorts_correctly() {
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

        let mut expected_order = vec![11, 20, 33, 0, 2, 10, 30, 3, 42, 105, 136, 1009];
        expected_order.sort();

        assert_eq!(actual_order, expected_order);
    }
}
