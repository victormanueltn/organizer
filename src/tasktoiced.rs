use crate::task::{self, Task};
use iced::alignment;
use iced::pure::{button, checkbox, row, text_input, widget::Button, widget::Text, Element};
use iced::Length;

pub(crate) trait TaskToIced {
    fn view(&self) -> Element<task::Message>;
}

fn add_button<'a>(text: &str, task_message: task::Message) -> Button<'a, task::Message> {
    let edit_text = Text::new(text)
        .width(Length::Units(60))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20);
    button(edit_text).on_press(task_message).padding(10)
}

impl TaskToIced for Task {
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

                let a_row = row()
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

                let a_row = row()
                    .spacing(20)
                    .align_items(iced::Alignment::Center)
                    .push(a_text_input);

                a_row.into()
            }
        }
    }
}
