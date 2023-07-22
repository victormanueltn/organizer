use crate::Time;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct PeriodicTask {
    description: String,
    initial_time: Time,
    frequency_in_hours: usize,
}

impl PeriodicTask {
    pub(crate) fn new(description: String, initial_time: Time, frequency_in_hours: usize) -> Self {
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

        let row = iced::widget::row(vec![])
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(text_input);

        row.into()
    }

    fn update(&mut self, message: Self::Message) {}
}
