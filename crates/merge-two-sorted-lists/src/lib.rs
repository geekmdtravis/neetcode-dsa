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

pub struct LikedList {
    pub head: Option<Box<ListNode>>,
}

impl LikedList {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LikedList { head: None }
    }

    #[allow(dead_code)]
    pub fn from_vec(vec: Vec<i32>) -> Self {
        let mut list = LikedList::new();
        for &val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = list.head.take();
            list.head = Some(node);
        }
        list
    }

    #[allow(dead_code)]
    pub fn to_vec(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            vec.push(node.val);
            current = &node.next;
        }
        vec
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
        let mut init_node = Box::new(ListNode::new(0));
        let mut cursor_new = &mut init_node;

        while let (Some(n1), Some(n2)) = (&list1, &list2) {
            if n1.val < n2.val {
                let mut node = list1.take().unwrap();
                list1 = node.next.take();
                cursor_new.next = Some(node);
            } else {
                let mut node = list2.take().unwrap();
                list2 = node.next.take();
                cursor_new.next = Some(node);
            }
            cursor_new = cursor_new.next.as_mut().unwrap();
        }
        cursor_new.next = list1.or(list2);

        init_node.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        let list1 = LikedList::from_vec(vec![1, 2, 4]);
        let list2 = LikedList::from_vec(vec![1, 3, 4]);
        let merged = Solution::merge_two_lists(list1.head, list2.head);
        let merged_list = LikedList { head: merged };
        assert_eq!(merged_list.to_vec(), vec![1, 1, 2, 3, 4, 4]);
    }
}
