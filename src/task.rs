pub struct Task {
    task_completed: bool,
    pub description: String,
}

impl Task {
    pub fn new(description: String) -> Task {
        Task {
            task_completed: false,
            description,
        }
    }

    pub fn completed(&self) -> bool {
        self.task_completed
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.task_completed = completed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_task_description() {
        let task = Task::new("This is a test task".to_string());
        assert_eq!("This is a test task", task.description);
    }

    #[test]
    fn empty_task_description() {
        let task = Task::new("".to_string());
        assert_eq!("", task.description);
    }
}
