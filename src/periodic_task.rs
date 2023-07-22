use crate::add_button;
use crate::Time;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct PeriodicTask {
    description: String,
    initial_time: Time,
    frequency_in_hours: Option<usize>,
}

impl PeriodicTask {
    pub(crate) fn new(
        description: String,
        initial_time: Time,
        frequency_in_hours: Option<usize>,
    ) -> Self {
        PeriodicTask {
            description,
            initial_time,
            frequency_in_hours,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TextInput(String),
    DeleteTask,
}

pub(crate) trait ToIced {
    type Message;
    fn view(&self) -> iced::Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
}

impl ToIced for PeriodicTask {
    type Message = Message;
    fn view(&self) -> iced::Element<Self::Message> {
        let text_input = iced::widget::text_input(
            "Describe your task...",
            &self.description,
            Self::Message::TextInput,
        );

        let delete_button =
            add_button("Delete", Message::DeleteTask).style(iced::theme::Button::Destructive);

        let row = iced::widget::row(vec![])
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(text_input)
            .push(delete_button);

        row.into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextInput(description) => self.description = description,
            Message::DeleteTask => panic!(), // Delete task is not used by PeriodicTask.
        };
    }
}
