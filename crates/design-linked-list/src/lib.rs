use std::{cell::RefCell, rc::Rc};

type NodeLink = Option<Rc<RefCell<ListNode>>>;

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

#[derive(Debug)]
struct MyLinkedList {
    length: i32,
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
            length: 0,
            head: None,
            tail: None,
        }
    }

    /// If the index is invalid, return -1.
    #[allow(dead_code)]
    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }

        let mut curr = match &self.head {
            Some(ln) => Rc::clone(ln),
            None => {
                return -1;
            }
        };

        for _ in 0..index {
            let next = match &curr.borrow().next {
                Some(ln) => Rc::clone(ln),
                None => {
                    return -1;
                }
            };
            curr = next;
        }
        curr.borrow().val
    }

    #[allow(dead_code)]
    fn add_at_head(&mut self, val: i32) {
        let mut new = ListNode::new(val);
        if self.length == 0 {
            let onlynode = &Rc::new(RefCell::new(new));
            self.head = Some(Rc::clone(onlynode));
            self.tail = Some(Rc::clone(onlynode));
            self.length = self.length + 1;
            return;
        };
        let head = match &self.head {
            Some(ln) => Rc::clone(ln),
            None => {
                return;
            }
        };
        new.next = Some(head);
        self.head = Some(Rc::new(RefCell::new(new)));
        self.length = self.length + 1;
    }

    #[allow(dead_code)]
    fn add_at_tail(&mut self, val: i32) {
        let new = Rc::new(RefCell::new(ListNode::new(val)));
        if let Some(tail) = self.tail.as_mut() {
            tail.borrow_mut().next = Some(new.clone());
            self.tail = Some(new);
        } else {
            self.head = Some(new.clone());
            self.tail = Some(new);
        }
        self.length = self.length + 1;
    }

    #[allow(dead_code)]
    fn add_at_index(&mut self, index: i32, val: i32) {
        let last_index = self.length - 1;

        if index > last_index {
            return;
        }

        if index == 0 {
            self.add_at_head(val);
            return;
        }

        if index == last_index {
            self.add_at_tail(val);
            return;
        }

        let mut curr = match &self.head {
            Some(ln) => Rc::clone(ln),
            None => {
                return;
            }
        };

        for _ in 0..index + 1 {
            let next = match &curr.borrow().next {
                Some(ln) => Rc::clone(ln),
                None => {
                    return;
                }
            };
            curr = next;
        }

        let right = match &curr.borrow().next {
            Some(ln) => Rc::clone(ln),
            None => {
                return;
            }
        };

        let mut new = ListNode::new(val);
        new.next = Some(right);

        let left = curr;
        left.borrow_mut().next = Some(Rc::new(RefCell::new(new)));
        self.length = self.length + 1;
    }

    #[allow(dead_code)]
    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index > self.length - 1 {
            return;
        }

        let mut curr = match &self.head {
            Some(ln) => Rc::clone(ln),
            None => {
                return;
            }
        };

        let mut left = None;
        for _ in 0..index {
            let next = match &curr.borrow().next {
                Some(ln) => Rc::clone(ln),
                None => {
                    return;
                }
            };
            left = Some(curr);
            curr = next;
        }

        let right = match &curr.borrow().next {
            Some(ln) => Rc::clone(ln),
            None => {
                return;
            }
        };

        curr.borrow_mut().next = None;
        if let Some(left) = left {
            left.borrow_mut().next = Some(right);
        };
        self.length = self.length - 1;
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
    fn it_adds_at_head() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_head(1);
        assert_eq!(linked_list.get(0), 1);
        linked_list.add_at_head(2);
        linked_list.add_at_head(3);
        assert_eq!(linked_list.get(0), 3);
        assert_eq!(linked_list.get(1), 2);
        assert_eq!(linked_list.get(2), 1);
    }

    #[test]
    fn it_adds_at_tail() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_tail(1);
        assert_eq!(linked_list.get(0), 1);
        linked_list.add_at_tail(2);
        linked_list.add_at_tail(3);
        assert_eq!(linked_list.get(0), 1);
        assert_eq!(linked_list.get(1), 2);
        assert_eq!(linked_list.get(2), 3);
    }

    #[test]
    fn it_deletes() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_head(1);
        println!("{:?}", linked_list);
        linked_list.add_at_index(0, 2);
        println!("{:?}", linked_list);
        linked_list.add_at_index(0, 3);
        println!("{:?}", linked_list);

        assert_eq!(linked_list.get(0), 3);
        assert_eq!(linked_list.get(1), 2);
        assert_eq!(linked_list.get(2), 1);

        linked_list.delete_at_index(1);
        println!("{:?}", linked_list);
        assert_eq!(linked_list.get(0), 3);
        assert_eq!(linked_list.get(1), 1);
    }

    #[test]
    fn it_adds_at_index_0() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_head(1);
        linked_list.add_at_index(0, 2);
        assert_eq!(linked_list.get(0), 2);
        assert_eq!(linked_list.get(1), 1);
    }

    #[test]
    fn it_adds_at_index_last() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_head(1);
        linked_list.add_at_head(0);
        linked_list.add_at_index(1, 2);

        assert_eq!(linked_list.get(0), 0);
        assert_eq!(linked_list.get(1), 2);
        assert_eq!(linked_list.get(2), 1);
    }

    #[test]
    fn it_adds_at_index_middle() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_head(3);
        linked_list.add_at_head(2);
        linked_list.add_at_head(1);
        linked_list.add_at_index(1, 4);

        assert_eq!(linked_list.get(0), 1);
        assert_eq!(linked_list.get(1), 4);
        assert_eq!(linked_list.get(2), 2);
        assert_eq!(linked_list.get(3), 3);
    }
}
