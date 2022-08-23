use iced::alignment;
use iced::pure::widget::Column;
use iced::pure::{button, checkbox, column, row, text_input, widget::Text, Element, Sandbox};
use iced::Length;
mod task;
use crate::task::{Task, TaskState};

pub struct Organizer {
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTaskCompletion(bool),
    EditingTask,
    TextInput(String),
    DescriptionEdited,
    AddTask,
}

impl Sandbox for Organizer {
    type Message = Message;

    fn new() -> Self {
        Organizer {
            tasks: vec![Task::new("A task to be completed.".to_string())],
        }
    }

    fn title(&self) -> String {
        String::from("Task")
    }

    fn view(&self) -> Element<Message> {
        let mut a_column = column();

        for task in self.tasks.iter() {
            a_column = add_task_view(&task, a_column);
        }

        a_column = add_task_button(a_column);

        a_column.into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTaskCompletion(completed) => self.tasks[0].set_completed(completed),
            Message::EditingTask => self.tasks[0].set_state(TaskState::BeingEdited),
            Message::TextInput(text) => self.tasks[0].edit(text),
            Message::DescriptionEdited => self.tasks[0].set_state(TaskState::Idle),
            Message::AddTask => self
                .tasks
                .push(Task::new("Yet another task for Victor!".to_string())),
        }
    }
}

fn add_task_view<'a>(a_task: &Task, a_column: Column<'a, Message>) -> Column<'a, Message> {
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

            let a_row = row()
                .spacing(20)
                .align_items(iced::Alignment::Center)
                .push(checkbox_instance)
                .push(edit_button);

            a_column
                .spacing(20)
                .align_items(iced::Alignment::Center)
                .push(a_row)
        }
        TaskState::BeingEdited => {
            let a_text_input = text_input(
                "Describe your task...",
                &a_task.description(),
                Message::TextInput,
            )
            .on_submit(Message::DescriptionEdited)
            .padding(10);

            a_column
                .spacing(20)
                .align_items(iced::Alignment::Center)
                .push(a_text_input)
        }
    }
}

fn add_task_button<'a>(a_row: Column<'a, Message>) -> Column<'a, Message> {
    let create_task_text = Text::new("Add a task for Victor.")
        .width(Length::Units(60))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20);
    let edit_button = button(create_task_text)
        .on_press(Message::AddTask)
        .padding(10);

    a_row
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .push(edit_button)
}
