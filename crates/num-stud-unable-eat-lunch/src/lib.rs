e std::collections::VecDeque;

struct Solution;

impl Solution {
    /// The school cafeteria offers circular and square sandwiches at
    /// lunch break, referred to by numbers 0 and 1 respectively.
    /// All students stand in a queue. Each student either prefers
    /// square or circular sandwiches.
    ///
    /// The number of sandwiches in the cafeteria is equal to the number
    /// of students. The sandwiches are placed in a stack. At each step:
    ///
    /// If the student at the front of the queue prefers the sandwich
    /// on the top of the stack, they will take it and leave the queue.
    /// Otherwise, they will leave it and go to the queue's end.
    /// This continues until none of the queue students want to take the
    /// top sandwich and are thus unable to eat.
    ///
    /// You are given two integer arrays students and sandwiches where
    /// sandwiches[i] is the type of the ith sandwich in the
    /// stack (i = 0 is the top of the stack) and students[j] is the
    /// preference of the jth student in the initial queue
    /// (j = 0 is the front of the queue). Return the number
    /// of students that are unable to eat.
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        if students.len() == 0 || sandwiches.len() == 0 {
            return 0;
        }

        let mut stud_queue = VecDeque::from(students);
        let mut sand_queue = Vec::from(sandwiches);

        let mut refusals = 0;
        loop {
            if refusals == stud_queue.len() {
                break;
            }
            let stud = stud_queue.pop_front().unwrap();
            if &stud == sand_queue.get(0).unwrap() {
                sand_queue.remove(0);
                refusals = 0;
            } else {
                stud_queue.push_back(stud);
                refusals += 1;
            }
        }
        stud_queue.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_students_get_preferred_sandwiches() {
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![1, 1, 0, 0];
        assert_eq!(Solution::count_students(students, sandwiches), 0);
    }

    #[test]
    fn some_students_unable_to_eat() {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 0, 0, 0, 1, 1];
        assert_eq!(Solution::count_students(students, sandwiches), 3);
    }

    #[test]
    fn no_students_get_preferred_sandwiches() {
        let students = vec![1, 1, 1];
        let sandwiches = vec![0, 0, 0];
        assert_eq!(Solution::count_students(students, sandwiches), 3);
    }

    #[test]
    fn single_student_gets_preferred() {
        let students = vec![0];
        let sandwiches = vec![0];
        assert_eq!(Solution::count_students(students, sandwiches), 0);
    }

    #[test]
    fn single_student_unable_to_eat() {
        let students = vec![1];
        let sandwiches = vec![0];
        assert_eq!(Solution::count_students(students, sandwiches), 1);
    }

    #[test]
    fn students_rotate_until_preferred_found() {
        let students = vec![0, 1, 0];
        let sandwiches = vec![0, 1, 0];
        assert_eq!(Solution::count_students(students, sandwiches), 0);
    }

    #[test]
    fn complex_rotation_scenario() {
        let students = vec![1, 0, 1, 0, 1];
        let sandwiches = vec![0, 1, 0, 1, 0];
        assert_eq!(Solution::count_students(students, sandwiches), 1);
    }

    #[test]
    fn empty_inputs() {
        let students = vec![];
        let sandwiches = vec![];
        assert_eq!(Solution::count_students(students, sandwiches), 0);
    }

    #[test]
    fn all_circular_sandwiches_all_circular_students() {
        let students = vec![0, 0, 0, 0];
        let sandwiches = vec![0, 0, 0, 0];
        assert_eq!(Solution::count_students(students, sandwiches), 0);
    }

    #[test]
    fn all_square_sandwiches_all_square_students() {
        let students = vec![1, 1, 1, 1];
        let sandwiches = vec![1, 1, 1, 1];
        assert_eq!(Solution::count_students(students, sandwiches), 0);
    }
}
