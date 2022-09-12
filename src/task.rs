pub struct Task {
    pub id: usize,
    task_completed: bool,
    description: String,
    state: State,
}

#[derive(Clone, Debug, Copy)]
pub(crate) enum State {
    Idle,
    BeingEdited,
}

#[derive(Debug, Clone)]
pub enum Message {
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
            state: State::Idle,
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

    pub(crate) fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub(crate) fn state(&self) -> State {
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

    #[test]
    fn completed_or_not_completed() {
        let mut task = Task::new(1, "".to_string());
        assert!(!task.completed());

        task.set_completed(false);
        assert!(!task.completed());

        task.set_completed(false);
        assert!(!task.completed());

        task.set_completed(true);
        assert!(task.completed());

        task.set_completed(true);
        assert!(task.completed());

        task.set_completed(false);
        assert!(!task.completed());
    }

    #[test]
    fn changing_state() {
        let mut task = Task::new(1, "".to_string());
        assert!(matches!(task.state(), State::Idle));

        task.set_state(State::Idle);
        assert!(matches!(task.state(), State::Idle));

        task.set_state(State::BeingEdited);
        assert!(matches!(task.state(), State::BeingEdited));

        task.set_state(State::Idle);
        assert!(matches!(task.state(), State::Idle));
    }
}
