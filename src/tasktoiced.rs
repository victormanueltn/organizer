use crate::task::{self, Task};
use crate::toiced::{add_button, ToIced};
use iced::widget::{checkbox, row, text_input};
use iced::Element;

impl ToIced for Task {
    type Message = task::Message;
    fn view(&self) -> Element<task::Message> {
        let checkbox_instance = checkbox(
            "".to_string(),
            self.completed(),
            task::Message::ToggleTaskCompletion,
        );

        let a_text_input = text_input(
            "Describe your task...",
            self.description(),
            task::Message::TextInput,
        )
        .padding(10);

        let delete_button =
            add_button("Delete", task::Message::DeleteTask).style(iced::theme::Button::Destructive);

        let a_row = row(vec![])
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(checkbox_instance)
            .push(a_text_input)
            .push(delete_button);

        a_row.into()
    }
}
