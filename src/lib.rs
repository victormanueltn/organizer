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
    AddTask,
    TaskMessage(usize, TaskMessage),
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    ToggleTaskCompletion(bool),
    EditingTask,
    TextInput(String),
    FinishedEdition,
}

impl Sandbox for Organizer {
    type Message = Message;

    fn new() -> Self {
        Organizer {
            tasks: vec![Task::new(0, "A task to be completed.".to_string())],
        }
    }

    fn title(&self) -> String {
        String::from("Task")
    }

    fn view(&self) -> Element<Message> {
        let mut a_column = column();

        for (index, task) in self.tasks.iter().enumerate() {
            a_column = a_column.push(
                add_task_view(&task).map(move |message| Message::TaskMessage(index, message)),
            );
        }

        a_column = add_task_button(a_column);

        a_column.spacing(10).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddTask => self.update_for_add_task(),
            Message::TaskMessage(task_id, task_message) => {
                self.update_for_task_message(task_id, task_message)
            }
        }
    }
}

fn add_task_view<'a>(task: &Task) -> Element<TaskMessage> {
    match task.state() {
        TaskState::Idle => {
            let checkbox_instance = checkbox(
                task.description().to_string(),
                task.completed(),
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
                &task.description(),
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

impl Organizer {
    pub fn update_for_add_task(&mut self) {
        self.tasks.push(Task::new(
            self.tasks.len(),
            "Yet another task for Victor!".to_string(),
        ))
    }

    pub fn update_for_task_message(&mut self, task_id: usize, task_message: TaskMessage) {
        if let Some(a_task) = self.tasks.get_mut(task_id) {
            match task_message {
                TaskMessage::ToggleTaskCompletion(completed) => a_task.set_completed(completed),
                TaskMessage::EditingTask => a_task.set_state(TaskState::BeingEdited),
                TaskMessage::TextInput(description) => a_task.edit(description),
                TaskMessage::FinishedEdition => a_task.set_state(TaskState::Idle),
            }
        }
    }
}
