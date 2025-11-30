#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct MyLinkedList {
    head: Option<Box<ListNode>>,
    tail: Option<Box<ListNode>>,
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

    #[allow(dead_code)]
    fn get(&self, index: i32) -> i32 {
        todo!("implement")
    }

    #[allow(dead_code)]
    fn add_at_head(&mut self, val: i32) {
        todo!("implement")
    }

    #[allow(dead_code)]
    fn add_at_tail(&mut self, val: i32) {
        todo!("implement")
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
