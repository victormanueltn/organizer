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
