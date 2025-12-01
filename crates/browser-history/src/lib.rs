use std::cmp::min;

/// Your BrowserHistory struct will be instantiated and called as such:
/// let obj = BrowserHistory::new(homepage);
/// obj.visit(url);
/// let ret_2: String = obj.back(steps);
/// let ret_3: String = obj.forward(steps);
struct BrowserHistory {
    idx_current: usize,
    visit_history: Vec<String>,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        let visit_history = vec![homepage];

        Self {
            idx_current: 0,
            visit_history,
        }
    }

    fn visit(&mut self, url: String) {
        self.visit_history = self.visit_history[0..self.idx_current + 1].to_vec();
        self.visit_history.push(url);
        self.idx_current += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.idx_current -= min(steps as usize, self.idx_current);
        self.visit_history.get(self.idx_current).unwrap().clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let max_fwd_steps = min(
            steps as usize,
            self.visit_history.len() - 1 - self.idx_current,
        );
        self.idx_current += max_fwd_steps;
        self.visit_history.get(self.idx_current).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_browser_history() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        // Test that we can create a new BrowserHistory
        // Implementation will be tested through other methods
    }

    #[test]
    fn test_single_visit() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        // Test that visit doesn't panic
    }

    #[test]
    fn test_multiple_visits() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.visit("facebook.com".to_string());
        history.visit("youtube.com".to_string());
        // Test multiple visits work
    }

    #[test]
    fn test_back_no_steps() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        let result = history.back(0);
        assert_eq!(result, "google.com");
    }

    #[test]
    fn test_back_one_step() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        let result = history.back(1);
        assert_eq!(result, "leetcode.com");
    }

    #[test]
    fn test_back_multiple_steps() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.visit("facebook.com".to_string());
        history.visit("youtube.com".to_string());

        let result = history.back(2);
        assert_eq!(result, "google.com");
    }

    #[test]
    fn test_back_beyond_start() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.visit("facebook.com".to_string());

        let result = history.back(10);
        assert_eq!(result, "leetcode.com");
    }

    #[test]
    fn test_forward_no_steps() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.back(1);
        let result = history.forward(0);
        assert_eq!(result, "leetcode.com");
    }

    #[test]
    fn test_forward_one_step() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.back(1);
        let result = history.forward(1);
        assert_eq!(result, "google.com");
    }

    #[test]
    fn test_forward_multiple_steps() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.visit("facebook.com".to_string());
        history.visit("youtube.com".to_string());
        history.back(3);

        let result = history.forward(2);
        assert_eq!(result, "facebook.com");
    }

    #[test]
    fn test_forward_beyond_end() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.visit("facebook.com".to_string());
        history.back(2);

        let result = history.forward(10);
        assert_eq!(result, "facebook.com");
    }

    #[test]
    fn test_back_and_forward_sequence() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.visit("facebook.com".to_string());
        history.visit("youtube.com".to_string());

        // Go back 2 steps
        let result1 = history.back(2);
        assert_eq!(result1, "google.com");

        // Go forward 1 step
        let result2 = history.forward(1);
        assert_eq!(result2, "facebook.com");

        // Go back 1 step
        let result3 = history.back(1);
        assert_eq!(result3, "google.com");
    }

    #[test]
    fn test_visit_after_navigation() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.visit("facebook.com".to_string());

        // Navigate back
        history.back(1);

        // Visit new URL should clear forward history
        history.visit("youtube.com".to_string());

        // Should not be able to go forward to facebook.com
        let result = history.forward(1);
        assert_eq!(result, "youtube.com");

        // Should be able to go back to google.com
        let result2 = history.back(1);
        assert_eq!(result2, "google.com");
    }

    #[test]
    fn test_no_forward_history_initially() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        let result = history.forward(5);
        assert_eq!(result, "leetcode.com");
    }

    #[test]
    fn test_no_back_history_with_single_page() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        let result = history.back(5);
        assert_eq!(result, "leetcode.com");
    }
}
