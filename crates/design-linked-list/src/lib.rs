type NodeLink = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: NodeLink,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct MyLinkedList {
    head: NodeLink,
    tail: NodeLink,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    #[allow(dead_code)]
    fn new() -> Self {
        MyLinkedList {
            head: None,
            tail: None,
        }
    }

    /// If the index is invalid, return -1.
    #[allow(dead_code)]
    fn get(&self, index: i32) -> i32 {
        let mut curr = match self.head.as_ref() {
            Some(n) => n,
            None => {
                return -1;
            }
        };

        for _ in 0..index {
            curr = match curr.next.as_ref() {
                Some(n) => n,
                None => {
                    return -1;
                }
            };
        }
        curr.val
    }

    #[allow(dead_code)]
    fn add_at_head(&mut self, val: i32) {
        let mut new_node = ListNode::new(val);
        new_node.next = self.head.take();
        self.head = Some(Box::new(new_node));
    }

    #[allow(dead_code)]
    fn add_at_tail(&mut self, val: i32) {
        let new = Box::new(ListNode::new(val));
        if let Some(tail) = self.tail.as_mut() {
            tail.next = Some(new);
            self.tail = new;
        } else {
            self.head = new;
            self.tail = new;
        }
    }

    #[allow(dead_code)]
    fn add_at_index(&mut self, index: i32, val: i32) {
        todo!("implement")
    }

    #[allow(dead_code)]
    fn delete_at_index(&self, index: i32) {
        todo!("Implement")
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 4);
    }
}
