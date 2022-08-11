use iced::alignment;
use iced::pure::widget::Row;
use iced::pure::{button, checkbox, row, widget::Text, Element, Sandbox};
use iced::Length;
mod task;
use crate::task::{Task, TaskState};

pub struct Organizer {
    task: Task,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ToggleTaskCompletion(bool),
    EditingTask,
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
        let a_row = row();

        let return_row = add_task_view(&self.task, a_row);

        return_row.into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTaskCompletion(completed) => self.task.set_completed(completed),
            Message::EditingTask => self.task.set_state(TaskState::BeingEdited),
        }
    }
}

fn add_task_view<'a>(a_task: &Task, a_row: Row<'a, Message>) -> Row<'a, Message> {
    match a_task.state() {
        TaskState::Idle => {
            let checkbox_instance = checkbox(
                a_task.description().to_string(),
                a_task.completed(),
                Message::ToggleTaskCompletion,
            );

            let edit_text = Text::new("Edit")
                .width(Length::Units(60))
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(20);
            let edit_button = button(edit_text).on_press(Message::EditingTask).padding(10);

            a_row
                .spacing(20)
                .align_items(iced::Alignment::Center)
                .push(checkbox_instance)
                .push(edit_button)
        }
        TaskState::BeingEdited => {
            let checkbox_instance = checkbox(
                a_task.description().to_string(),
                a_task.completed(),
                Message::ToggleTaskCompletion,
            );

            let edit_text = Text::new("Pressed Edit")
                .width(Length::Units(60))
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(20);
            let edit_button = button(edit_text).on_press(Message::EditingTask).padding(10);

            a_row
                .spacing(20)
                .align_items(iced::Alignment::Center)
                .push(checkbox_instance)
                .push(edit_button)
        }
    }
}
