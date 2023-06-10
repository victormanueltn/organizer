mod data;
use crate::views::{ListView, SummaryView};
mod datatoiced;
mod task;
mod tasktoiced;
mod time;
mod toiced;
mod views;
use std::vec;

use crate::toiced::add_button;
use crate::views::{Message, ViewType};
use data::{Data, Filters};
use iced::widget::Text;
use iced::Element;
use iced::Sandbox;
use task::Task;
use time::{Time, TimeError};

pub struct Organizer {
    data: Data,
    error_text: Option<String>,
    file_name: String,
    view_type: Option<ViewType>,
    summary_dates: SummaryDates,
}

struct SummaryDates {
    initial_day: u32,
    initial_month: u32,
    initial_year: u32,
    initial_date: Result<Time, TimeError>,
    final_day: u32,
    final_month: u32,
    final_year: u32,
    final_date: Result<Time, TimeError>,
}

#[cfg(not(tarpaulin_include))]
impl Sandbox for Organizer {
    type Message = Message;

    fn new() -> Self {
        let today = Time::now();
        Organizer {
            data: Data {
                tasks: vec![],
                filters: Filters {
                    todo: true,
                    complete: false,
                },
            },
            error_text: None,
            file_name: String::new(),
            view_type: Some(ViewType::List),
            summary_dates: SummaryDates {
                initial_day: 1,
                initial_month: 1,
                initial_year: 2023,
                initial_date: Time::new(1, 1, 2023, 0, 0, 0),
                final_day: today.day(),
                final_month: today.month(),
                final_year: today.year(),
                final_date: Time::new(today.day(), today.month(), today.year(), 0, 0, 0),
            },
        }
    }

    fn title(&self) -> String {
        String::from("Organizer")
    }

    fn view(&self) -> Element<Message> {
        match self.view_type.unwrap() {
            ViewType::List => self.view_as_list().map(Message::ListViewMessage),
            ViewType::Summary => self.view_as_summary().map(Message::SummaryViewMessage),
        }
    }

    fn update(&mut self, message: Message) {
        self.error_text = None;
        match message {
            Message::ListViewMessage(message) => self.update_list_view(message),
            Message::SummaryViewMessage(message) => self.update_summary_view(message),
        }
    }
}

impl Organizer {
    pub fn add_task(&mut self) {
        self.data.tasks.push(Task::new(self.data.tasks.len()))
    }

    pub fn process_task_message(&mut self, task_id: usize, task_message: task::Message) {
        if let Some(a_task) = self.data.tasks.get_mut(task_id) {
            match task_message {
                task::Message::ToggleTaskCompletion(completed) => a_task.set_completed(completed),
                task::Message::TextInput(description) => a_task.edit(&description),
                task::Message::DeleteTask => {
                    self.data.tasks.remove(task_id);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_title() {
        let organizer = Organizer::new();
        assert_eq!(organizer.title(), "Organizer");
    }

    #[test]
    fn add_task() {
        let mut organizer = Organizer::new();
        assert_eq!(organizer.data.tasks.len(), 0);

        organizer.add_task();
        assert_eq!(organizer.data.tasks.len(), 1);

        organizer.add_task();
        organizer.add_task();
        organizer.add_task();
        assert_eq!(organizer.data.tasks.len(), 4);
    }

    mod process_task_message {
        use super::*;

        #[test]
        fn toggle_task_completion() {
            let mut organizer = Organizer::new();
            organizer.add_task();

            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(!task.completed());
            }

            organizer.process_task_message(0, task::Message::ToggleTaskCompletion(false));
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(!task.completed());
            }

            organizer.process_task_message(0, task::Message::ToggleTaskCompletion(true));
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(task.completed());
            }

            organizer.process_task_message(0, task::Message::ToggleTaskCompletion(false));
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(!task.completed());
            }
        }

        #[test]
        fn text_input() {
            let mut organizer = Organizer::new();
            organizer.add_task();

            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert_eq!(task.description(), "");
            }

            organizer
                .process_task_message(0, task::Message::TextInput("A description".to_string()));
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert_eq!(task.description(), "A description");
            }
        }

        #[test]
        fn delete_task() {
            let mut organizer = Organizer::new();
            organizer.add_task();
            organizer.add_task();
            assert_eq!(organizer.data.tasks.len(), 2);

            organizer.process_task_message(0, task::Message::DeleteTask);
            assert_eq!(organizer.data.tasks.len(), 1);

            organizer.process_task_message(0, task::Message::DeleteTask);
            assert_eq!(organizer.data.tasks.len(), 0);

            organizer.process_task_message(1, task::Message::DeleteTask);
            assert_eq!(organizer.data.tasks.len(), 0);
        }
    }

    mod update {
        use super::*;
        use crate::views::{ListMessage};

        #[test]
        fn add_task() {
            let mut organizer = Organizer::new();
            organizer.update(Message::ListViewMessage(ListMessage::AddTask));
            assert_eq!(organizer.data.tasks.len(), 1);
        }

        #[test]
        #[should_panic]
        fn message_to_inexisting_task() {
            let mut organizer = Organizer::new();
            organizer.update(Message::ListViewMessage(ListMessage::AddTask));

            organizer.update(Message::ListViewMessage(ListMessage::Task(
                1,
                task::Message::DeleteTask,
            )));
            assert_eq!(organizer.data.tasks.len(), 2);
        }

        #[test]
        fn task_message() {
            let mut organizer = Organizer::new();

            organizer.update(Message::ListViewMessage(ListMessage::AddTask));
            organizer.update(Message::ListViewMessage(ListMessage::AddTask));
            organizer.update(Message::ListViewMessage(ListMessage::AddTask));
            assert_eq!(organizer.data.tasks.len(), 3);

            organizer.update(Message::ListViewMessage(ListMessage::Task(
                0,
                task::Message::TextInput("A".to_string()),
            )));
            organizer.update(Message::ListViewMessage(ListMessage::Task(
                1,
                task::Message::TextInput("B".to_string()),
            )));
            organizer.update(Message::ListViewMessage(ListMessage::Task(
                2,
                task::Message::TextInput("C".to_string()),
            )));

            organizer.update(Message::ListViewMessage(ListMessage::Task(
                1,
                task::Message::DeleteTask,
            )));
            assert_eq!(organizer.data.tasks.len(), 2);

            assert_eq!(organizer.data.tasks[0].description(), "A");
            assert_eq!(organizer.data.tasks[1].description(), "C");
        }
    }
}
