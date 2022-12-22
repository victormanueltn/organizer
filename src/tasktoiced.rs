use crate::task::{self, Task};
use crate::toiced::{add_button, ToIced};
use iced::widget::{checkbox, row, text_input};
use iced::Element;

impl ToIced for Task {
    type Message = task::Message;
    fn view(&self) -> Element<task::Message> {
        match self.state() {
            task::State::Idle => {
                let checkbox_instance = checkbox(
                    self.description().to_string(),
                    self.completed(),
                    task::Message::ToggleTaskCompletion,
                );

                let edit_button = add_button("Edit", task::Message::EditTask);
                let delete_button = add_button("Delete", task::Message::DeleteTask);

                let a_row = row(vec![])
                    .spacing(20)
                    .align_items(iced::Alignment::Center)
                    .push(checkbox_instance)
                    .push(edit_button)
                    .push(delete_button);

                a_row.into()
            }
            task::State::BeingEdited => {
                let a_text_input = text_input(
                    "Describe your task...",
                    self.description(),
                    task::Message::TextInput,
                )
                .padding(10)
                .on_submit(task::Message::FinishedEdition);

                let a_row = row(vec![])
                    .spacing(20)
                    .align_items(iced::Alignment::Center)
                    .push(a_text_input);

                a_row.into()
            }
        }
    }
}
