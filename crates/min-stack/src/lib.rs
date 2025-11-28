/// Design a stack that supports push, pop, top, and retrieving
/// the minimum element in constant time.
///
/// The object should support the following operations:
/// - `push(val)` -- Pushes the element val onto the stack.
/// - `pop()` -- Removes the element on the top of the stack.
/// - `top()` -- Gets the top element of the stack.
/// - `get_min()` -- Retrieves the minimum element in the stack.
///
struct MinStack {
    /// Two-tuple, first is val and second is min.
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    #[allow(dead_code)]
    fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    #[allow(dead_code)]
    fn push(&mut self, val: i32) {
        if self.stack.len() == 0 {
            self.stack.push((val, val));
        } else {
            let min = if val < self.get_min() {
                val
            } else {
                self.get_min()
            };
            self.stack.push((val, min));
        }
    }

    #[allow(dead_code)]
    fn pop(&mut self) {
        self.stack.pop().unwrap();
    }

    #[allow(dead_code)]
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    #[allow(dead_code)]
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&["push","push","push","getMin","pop","top","getMin"], &[Some(-2), Some(0), Some(-3), None, None, None, None])]
    fn it_works(operations: &[&str], values: &[Option<i32>]) {
        let mut s = MinStack::new();
        for (o, v) in std::iter::zip(operations, values) {
            match *o {
                "push" => {
                    s.push(v.unwrap());
                }
                "pop" => {
                    s.pop();
                }
                "top" => {
                    s.top();
                }
                "getMin" => {
                    s.get_min();
                }
                _ => panic!("Not implemented"),
            }
        }
    }
}
