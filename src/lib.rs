use iced::Sandbox;
mod task;
use crate::task::Task;

#[derive(Default)]
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
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Task")
    }

    fn view(&mut self) -> iced::Element<Message> {
        iced::Column::new()
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(iced::Checkbox::new(
                self.task.completed(),
                String::from(""),
                Message::TaskCompleted,
            ))
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TaskCompleted(completed) => self.task.set_completed(completed),
        }
    }
}
