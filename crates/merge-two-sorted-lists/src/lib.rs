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
