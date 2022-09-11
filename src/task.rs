pub struct Task {
    pub id: usize,
    task_completed: bool,
    description: String,
    state: TaskState,
}

#[derive(Clone, Copy)]
pub(crate) enum TaskState {
    Idle,
    BeingEdited,
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    ToggleTaskCompletion(bool),
    EditTask,
    TextInput(String),
    FinishedEdition,
    DeleteTask,
}

impl Task {
    pub fn new(id: usize, description: String) -> Task {
        Task {
            id,
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

    pub(crate) fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    pub(crate) fn state(&self) -> TaskState {
        self.state
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
        let task = Task::new(1, "This is a test task".to_string());
        assert_eq!("This is a test task", task.description());
    }

    #[test]
    fn empty_task_description() {
        let task = Task::new(1, "".to_string());
        assert_eq!("", task.description());
    }

    #[test]
    fn edit_description() {
        let mut task = Task::new(1, "This is a test task".to_string());

        task.edit("Edited task description".to_string());
        assert_eq!("Edited task description", task.description());
    }

    #[test]
    fn edit_to_empty() {
        let mut task = Task::new(1, "This is a test task".to_string());

        task.edit("".to_string());
        assert_eq!("", task.description());
    }

    #[test]
    fn edit_from_empty() {
        let mut task = Task::new(1, "".to_string());

        task.edit("A description".to_string());
        assert_eq!("A description", task.description());
    }
}
