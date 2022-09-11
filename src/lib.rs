use iced::alignment;
use iced::pure::widget::Column;
use iced::pure::{button, column, widget::Text, Element, Sandbox};
use iced::Length;
mod task;
use task::{Task, TaskMessage, TaskState};
mod tasktoiced;
use tasktoiced::TaskToIced;

pub struct Organizer {
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddTask,
    TaskMessage(usize, TaskMessage),
}

impl Sandbox for Organizer {
    type Message = Message;

    fn new() -> Self {
        Organizer { tasks: vec![] }
    }

    fn title(&self) -> String {
        String::from("Organizer")
    }

    fn view(&self) -> Element<Message> {
        let mut a_column = column();

        for (index, task) in self.tasks.iter().enumerate() {
            a_column = a_column.push(
                task.view()
                    .map(move |message| Message::TaskMessage(index, message)),
            );
        }

        a_column = Organizer::add_task_button(a_column);

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

impl Organizer {
    pub fn update_for_add_task(&mut self) {
        self.tasks.push(Task::new(self.tasks.len(), "".to_string()))
    }

    pub fn update_for_task_message(&mut self, task_id: usize, task_message: TaskMessage) {
        if let Some(a_task) = self.tasks.get_mut(task_id) {
            match task_message {
                TaskMessage::ToggleTaskCompletion(completed) => a_task.set_completed(completed),
                TaskMessage::EditTask => a_task.set_state(TaskState::BeingEdited),
                TaskMessage::TextInput(description) => a_task.edit(description),
                TaskMessage::FinishedEdition => a_task.set_state(TaskState::Idle),
                TaskMessage::DeleteTask => {
                    self.tasks.remove(task_id);
                }
            }
        }
    }

    pub fn add_task_button<'a>(a_column: Column<'a, Message>) -> Column<'a, Message> {
        let create_task_text = Text::new("Add a new task")
            .width(Length::Units(120))
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(20);
        let edit_button = button(create_task_text)
            .on_press(Message::AddTask)
            .padding(10);

        a_column
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .push(edit_button)
    }
}
