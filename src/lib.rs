use iced::alignment;
use iced::pure::container;
use iced::pure::widget::Column;
use iced::pure::{button, column, widget::Text, Element, Sandbox};
use iced::Length;
mod task;
use task::Task;
mod tasktoiced;
use tasktoiced::TaskToIced;
mod data;
use data::Data;

pub struct Organizer {
    data: Data,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddTask,
    TaskMessage(usize, task::Message),
}

#[cfg(not(tarpaulin_include))]
impl Sandbox for Organizer {
    type Message = Message;

    fn new() -> Self {
        Organizer {
            data: Data { tasks: vec![] },
        }
    }

    fn title(&self) -> String {
        String::from("Organizer")
    }

    #[cfg(not(tarpaulin_include))]
    fn view(&self) -> Element<Message> {
        let mut a_column = column();

        for (index, task) in self.data.tasks.iter().enumerate() {
            a_column = a_column.push(
                task.view()
                    .map(move |message| Message::TaskMessage(index, message)),
            );
        }

        a_column = Organizer::add_task_button(a_column)
            .spacing(10)
            .align_items(iced::Alignment::Start);

        container(a_column).width(Length::Fill).center_x().into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddTask => self.add_task(),
            Message::TaskMessage(task_id, task_message) => {
                if task_id > self.data.tasks.len() {
                    panic!("Tried to update inexisting task.")
                };
                self.process_task_message(task_id, task_message)
            }
        }
    }
}

impl Organizer {
    pub fn add_task(&mut self) {
        self.data.tasks.push(Task::new(self.data.tasks.len()))
    }

    pub fn process_task_message(&mut self, task_id: usize, task_message: task::Message) {
        if let Some(a_task) = self.data.tasks.get_mut(task_id) {
            match task_message {
                task::Message::ToggleTaskCompletion(completed) => a_task.set_completed(completed),
                task::Message::EditTask => a_task.set_state(task::State::BeingEdited),
                task::Message::TextInput(description) => a_task.edit(&description),
                task::Message::FinishedEdition => a_task.set_state(task::State::Idle),
                task::Message::DeleteTask => {
                    self.data.tasks.remove(task_id);
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
    fn check_title() {
        let organizer = Organizer::new();
        assert_eq!(organizer.title(), "Organizer");
    }

    #[test]
    fn add_task() {
        let mut organizer = Organizer::new();
        assert_eq!(organizer.data.tasks.len(), 0);

        organizer.add_task();
        assert_eq!(organizer.data.tasks.len(), 1);

        organizer.add_task();
        organizer.add_task();
        organizer.add_task();
        assert_eq!(organizer.data.tasks.len(), 4);
    }

    mod process_task_message {
        use super::*;

        #[test]
        fn toggle_task_completion() {
            let mut organizer = Organizer::new();
            organizer.add_task();

            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(!task.completed());
            }

            organizer.process_task_message(0, task::Message::ToggleTaskCompletion(false));
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(!task.completed());
            }

            organizer.process_task_message(0, task::Message::ToggleTaskCompletion(true));
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(task.completed());
            }

            organizer.process_task_message(0, task::Message::ToggleTaskCompletion(false));
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(!task.completed());
            }
        }

        #[test]
        fn edit_task() {
            let mut organizer = Organizer::new();
            organizer.add_task();

            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(matches!(task.state(), task::State::Idle));
            }

            organizer.process_task_message(0, task::Message::EditTask);
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(matches!(task.state(), task::State::BeingEdited));
            }
        }

        #[test]
        fn text_input() {
            let mut organizer = Organizer::new();
            organizer.add_task();

            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert_eq!(task.description(), "");
            }

            organizer
                .process_task_message(0, task::Message::TextInput("A description".to_string()));
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert_eq!(task.description(), "A description");
            }
        }

        #[test]
        fn finished_edition() {
            let mut organizer = Organizer::new();
            organizer.add_task();
            organizer.process_task_message(0, task::Message::EditTask);
            organizer.process_task_message(0, task::Message::FinishedEdition);
            {
                let task = organizer.data.tasks.get(0).unwrap();
                assert!(matches!(task.state(), task::State::Idle));
            }
        }

        #[test]
        fn delete_task() {
            let mut organizer = Organizer::new();
            organizer.add_task();
            organizer.add_task();
            assert_eq!(organizer.data.tasks.len(), 2);

            organizer.process_task_message(0, task::Message::DeleteTask);
            assert_eq!(organizer.data.tasks.len(), 1);

            organizer.process_task_message(0, task::Message::DeleteTask);
            assert_eq!(organizer.data.tasks.len(), 0);

            organizer.process_task_message(1, task::Message::DeleteTask);
            assert_eq!(organizer.data.tasks.len(), 0);
        }
    }

    mod update {
        use super::*;

        #[test]
        fn add_task() {
            let mut organizer = Organizer::new();
            organizer.update(Message::AddTask);
            assert_eq!(organizer.data.tasks.len(), 1);
        }

        #[test]
        #[should_panic]
        fn message_to_inexisting_task() {
            let mut organizer = Organizer::new();
            organizer.update(Message::AddTask);

            organizer.update(Message::TaskMessage(1, task::Message::DeleteTask));
            assert_eq!(organizer.data.tasks.len(), 2);
        }

        #[test]
        fn task_message() {
            let mut organizer = Organizer::new();

            organizer.update(Message::AddTask);
            organizer.update(Message::AddTask);
            organizer.update(Message::AddTask);
            assert_eq!(organizer.data.tasks.len(), 3);

            organizer.update(Message::TaskMessage(0, task::Message::EditTask));
            organizer.update(Message::TaskMessage(1, task::Message::EditTask));
            organizer.update(Message::TaskMessage(2, task::Message::EditTask));

            organizer.data.tasks.iter().for_each(|task| {
                assert!(matches!(task.state(), task::State::BeingEdited));
            });

            organizer.update(Message::TaskMessage(
                0,
                task::Message::TextInput("A".to_string()),
            ));
            organizer.update(Message::TaskMessage(
                1,
                task::Message::TextInput("B".to_string()),
            ));
            organizer.update(Message::TaskMessage(
                2,
                task::Message::TextInput("C".to_string()),
            ));

            organizer.update(Message::TaskMessage(0, task::Message::FinishedEdition));
            organizer.update(Message::TaskMessage(1, task::Message::FinishedEdition));
            organizer.update(Message::TaskMessage(2, task::Message::FinishedEdition));

            organizer.data.tasks.iter().for_each(|task| {
                assert!(matches!(task.state(), task::State::Idle));
            });

            organizer.update(Message::TaskMessage(1, task::Message::DeleteTask));
            assert_eq!(organizer.data.tasks.len(), 2);

            assert_eq!(organizer.data.tasks[0].description(), "A");
            assert_eq!(organizer.data.tasks[1].description(), "C");
        }
    }
}
