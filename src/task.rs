use crate::time::Time;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Task {
    pub id: usize,
    task_completed: bool,
    description: String,
    creation_time: Time,
    completion_time: Option<Time>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTaskCompletion(bool),
    TextInput(String),
    DeleteTask,
}

impl Task {
    pub fn new(id: usize) -> Task {
        Task {
            id,
            task_completed: false,
            description: "".to_string(),
            creation_time: Time::now(),
            completion_time: None,
        }
    }

    pub fn completed(&self) -> bool {
        self.task_completed
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.task_completed = completed;
    }

    pub fn edit(&mut self, description: &str) {
        self.description = description.to_string();
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
        let mut task = Task::new(1);

        task.edit("This is a test task");
        assert_eq!("This is a test task", task.description());
    }

    #[test]
    fn empty_task_description() {
        let task = Task::new(1);
        assert_eq!("", task.description());
    }

    #[test]
    fn edit_description() {
        let mut task = Task::new(1);
        task.edit("This is a test task");

        task.edit("Edited task description");
        assert_eq!("Edited task description", task.description());
    }

    #[test]
    fn edit_to_empty() {
        let mut task = Task::new(1);
        task.edit("This is a test task");

        task.edit("");
        assert_eq!("", task.description());
    }

    #[test]
    fn edit_from_empty() {
        let mut task = Task::new(1);

        task.edit("A description");
        assert_eq!("A description", task.description());
    }

    #[test]
    fn completed_or_not_completed() {
        let mut task = Task::new(1);
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
}
