use crate::tasktoiced::FADE_OUT_TIME;
use crate::time::{Duration, Time};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct Task {
    pub id: usize,
    task_completed: bool,
    description: String,
    pub creation_time: Time,
    pub completion_time: Option<Time>,
    #[serde(default)]
    pub snooze_information: SnoozeInformation,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTaskCompletion(bool),
    TextInput(String),
    DeleteTask,
    AddSnoozeTime,
    SetSnoozeDuration(SnoozeDuration),
    SetSnoozeQuantity(String),
    Unsnooze,
}

#[derive(Debug, Clone)]
pub enum SnoozeDuration {
    Hour,
    Day,
    Week,
    Month,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub(crate) struct SnoozeInformation {
    pub visible: bool,
    pub quantity: u32,
    pub snooze_until: Option<Time>,
}

impl Task {
    pub fn new(id: usize) -> Task {
        Task {
            id,
            task_completed: false,
            description: "".to_string(),
            creation_time: Time::now(),
            completion_time: None,
            snooze_information: SnoozeInformation {
                quantity: 1,
                visible: false,
                snooze_until: None,
            },
        }
    }

    pub fn completed(&self) -> bool {
        self.task_completed
    }

    pub fn visible_as_pending(&self) -> bool {
        if !self.task_completed {
            true
        } else if let Some(completion_time) = self.completion_time.as_ref() {
            &Time::now() - completion_time < Duration::from_minutes(FADE_OUT_TIME)
                && self.task_completed
        } else {
            true
        }
    }

    pub fn hidden_because_of_snooze(&self) -> bool {
        self.snooze_information
            .snooze_until
            .as_ref()
            .is_some_and(|snooze_until| snooze_until > &Time::now())
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completion_time = Some(Time::now());
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

    #[test]
    fn set_completion_time() {
        let mut task = Task::new(1);
        assert!(task.completion_time.is_none());

        task.set_completed(true);
        assert!(task.completion_time.is_some());
    }

    #[test]
    fn visible_as_pending_test() {
        let mut task = Task::new(1);

        task.set_completed(false);
        assert!(task.visible_as_pending());

        task.set_completed(true);
        task.completion_time = Option::Some(Time::now());
        assert!(task.visible_as_pending());

        let far_away_time = Time::from("Sat, 21 Jan 2023 12:25:20 +0100");
        task.completion_time = Option::Some(far_away_time);
        assert!(!task.visible_as_pending());
    }
}
