use iced::pure::{checkbox, column, Element, Sandbox};
mod task;
use crate::task::Task;

pub struct Organizer {
    task: Task,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    TaskCompleted(bool),
}

impl Sandbox for Organizer {
    type Message = Message;

    fn new() -> Self {
        Organizer {
            task: Task::new("A task to be completed.".to_string()),
        }
    }

    fn title(&self) -> String {
        String::from("Task")
    }

    fn view(&self) -> Element<Message> {
        let checkbox = checkbox(
            &self.task.description,
            self.task.completed(),
            Message::TaskCompleted,
        );

        column()
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(checkbox)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TaskCompleted(completed) => self.task.set_completed(completed),
        }
    }
}
