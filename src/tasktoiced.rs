use crate::Task;
use crate::TaskMessage;
use crate::TaskState;
use iced::alignment;
use iced::pure::{button, checkbox, row, text_input, widget::Text, Element};
use iced::Length;

pub(crate) trait TaskToIced {
    fn view<'a>(&self) -> Element<TaskMessage>;
}

impl TaskToIced for Task {
    fn view<'a>(&self) -> Element<TaskMessage> {
        match self.state() {
            TaskState::Idle => {
                let checkbox_instance = checkbox(
                    self.description().to_string(),
                    self.completed(),
                    TaskMessage::ToggleTaskCompletion,
                );

                let edit_text = Text::new("Edit")
                    .width(Length::Units(60))
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(20);
                let edit_button = button(edit_text)
                    .on_press(TaskMessage::EditingTask)
                    .padding(10);

                let a_row = row()
                    .spacing(20)
                    .align_items(iced::Alignment::Center)
                    .push(checkbox_instance)
                    .push(edit_button);

                a_row.into()
            }
            TaskState::BeingEdited => {
                let a_text_input = text_input(
                    "Describe your task...",
                    &self.description(),
                    TaskMessage::TextInput,
                )
                .padding(10)
                .on_submit(TaskMessage::FinishedEdition);

                let a_row = row()
                    .spacing(20)
                    .align_items(iced::Alignment::Center)
                    .push(a_text_input);

                a_row.into()
            }
        }
    }
}
