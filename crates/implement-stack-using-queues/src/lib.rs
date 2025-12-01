/// Implement a last-in-first-out (LIFO) stack using only two queues.
/// The implemented stack should support all the functions of a
/// normal stack (push, top, pop, and empty).
///
/// Implement the MyStack class:
///
/// void push(int x) Pushes element x to the top of the stack.
/// int pop() Removes the element on the top of the stack and returns it.
/// int top() Returns the element on the top of the stack.
/// boolean empty() Returns true if the stack is empty, false otherwise.
///
/// Notes:
/// You must use only standard operations of a queue, which means
/// that only push to back, peek/pop from front, size and is empty
/// operations are valid.
///
/// Depending on your language, the queue may not be supported
/// natively. You may simulate a queue using a list or deque
/// (double-ended queue) as long as you use only a queue's
/// standard operations.
///
struct MyStack {
    stack: Vec<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        let idx_top = self.stack.len() - 1;
        let val = self.stack.remove(idx_top);
        val
    }

    fn top(&self) -> i32 {
        let idx_top = self.stack.len() - 1;
        *self.stack.get(idx_top).unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack = MyStack::new();
        assert!(stack.empty());
    }

    #[test]
    fn test_push_and_top() {
        let mut stack = MyStack::new();
        stack.push(1);
        assert_eq!(stack.top(), 1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        stack.push(3);
        assert_eq!(stack.top(), 3);
    }

    #[test]
    fn test_pop() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), 3);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
    }

    #[test]
    fn test_empty_after_operations() {
        let mut stack = MyStack::new();
        assert!(stack.empty());

        stack.push(42);
        assert!(!stack.empty());

        stack.pop();
        assert!(stack.empty());
    }

    #[test]
    fn test_multiple_operations() {
        let mut stack = MyStack::new();

        for i in 1..=5 {
            stack.push(i);
        }

        assert_eq!(stack.top(), 5);
        assert_eq!(stack.pop(), 5);
        assert_eq!(stack.top(), 4);

        stack.push(10);
        assert_eq!(stack.top(), 10);
        assert_eq!(stack.pop(), 10);
        assert_eq!(stack.pop(), 4);

        assert!(!stack.empty());
    }
}
