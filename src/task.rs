#[derive(Default)]
pub struct Task {
    task_completed: bool,
}

impl Task {
    pub fn completed(&self) -> bool {
        self.task_completed
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.task_completed = completed;
    }
}
