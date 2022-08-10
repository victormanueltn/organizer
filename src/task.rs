#[derive(Debug, Clone, Copy)]
pub enum TaskMessage {
    TaskCompleted(bool),
    ButtonPressed,
}

pub struct Task {
    task_completed: bool,
    description: String,
    state: TaskState,
}

enum TaskState {
    Idle,
    BeingEdited,
}

impl Task {
    pub fn new(description: String) -> Task {
        Task {
            task_completed: false,
            description,
            state: TaskState::Idle,
        }
    }

    pub fn completed(&self) -> bool {
        self.task_completed
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.task_completed = completed;
    }

    pub fn edit(&mut self, description: String) {
        self.description = description;
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_task_description() {
        let task = Task::new("This is a test task".to_string());
        assert_eq!("This is a test task", task.description());
    }

    #[test]
    fn empty_task_description() {
        let task = Task::new("".to_string());
        assert_eq!("", task.description());
    }

    #[test]
    fn edit_description() {
        let mut task = Task::new("This is a test task".to_string());

        task.edit("Edited task description".to_string());
        assert_eq!("Edited task description", task.description());
    }

    #[test]
    fn edit_to_empty() {
        let mut task = Task::new("This is a test task".to_string());

        task.edit("".to_string());
        assert_eq!("", task.description());
    }

    #[test]
    fn edit_from_empty() {
        let mut task = Task::new("".to_string());

        task.edit("A description".to_string());
        assert_eq!("A description", task.description());
    }
}
