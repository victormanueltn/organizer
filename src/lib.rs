use iced::alignment;
use iced::pure::widget::Column;
use iced::pure::{button, column, widget::Text, Element, Sandbox};
use iced::Length;
mod task;
use task::Task;
mod tasktoiced;
use tasktoiced::TaskToIced;

pub struct Organizer {
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddTask,
    TaskMessage(usize, task::Message),
}

impl Sandbox for Organizer {
    type Message = Message;

    fn new() -> Self {
        Organizer { tasks: vec![] }
    }

    fn title(&self) -> String {
        String::from("Organizer")
    }

    #[cfg(not(tarpaulin_include))]
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
            Message::AddTask => self.add_task(),
            Message::TaskMessage(task_id, task_message) => {
                self.process_task_message(task_id, task_message)
            }
        }
    }
}

impl Organizer {
    pub fn add_task(&mut self) {
        self.tasks.push(Task::new(self.tasks.len()))
    }

    pub fn process_task_message(&mut self, task_id: usize, task_message: task::Message) {
        if let Some(a_task) = self.tasks.get_mut(task_id) {
            match task_message {
                task::Message::ToggleTaskCompletion(completed) => a_task.set_completed(completed),
                task::Message::EditTask => a_task.set_state(task::State::BeingEdited),
                task::Message::TextInput(description) => a_task.edit(&description),
                task::Message::FinishedEdition => a_task.set_state(task::State::Idle),
                task::Message::DeleteTask => {
                    self.tasks.remove(task_id);
                }
            }
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn add_task_button(a_column: Column<Message>) -> Column<Message> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_task() {
        let mut organizer = Organizer::new();
        assert_eq!(organizer.tasks.len(), 0);

        organizer.add_task();
        assert_eq!(organizer.tasks.len(), 1);

        organizer.add_task();
        organizer.add_task();
        organizer.add_task();
        assert_eq!(organizer.tasks.len(), 4);
    }

    mod process_task_message {
        use super::*;

        #[test]
        fn process_task_message_toggle_task_completion() {
            let mut organizer = Organizer::new();
            organizer.add_task();

            {
                let task = organizer.tasks.get(0).unwrap();
                assert!(!task.completed());
            }

            organizer.process_task_message(0, task::Message::ToggleTaskCompletion(false));
            {
                let task = organizer.tasks.get(0).unwrap();
                assert!(!task.completed());
            }

            organizer.process_task_message(0, task::Message::ToggleTaskCompletion(true));
            {
                let task = organizer.tasks.get(0).unwrap();
                assert!(task.completed());
            }

            organizer.process_task_message(0, task::Message::ToggleTaskCompletion(false));
            {
                let task = organizer.tasks.get(0).unwrap();
                assert!(!task.completed());
            }
        }
    }
}
