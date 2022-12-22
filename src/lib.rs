mod data;
mod task;
mod tasktoiced;
use data::{Data, Message};
mod datatoiced;
mod toiced;
use iced::widget::column;
use iced::Element;
use iced::Sandbox;
use task::Task;
use toiced::ToIced;

pub struct Organizer {
    data: Data,
    error_text: Option<String>,
}

#[cfg(not(tarpaulin_include))]
impl Sandbox for Organizer {
    type Message = data::Message;

    fn new() -> Self {
        Organizer {
            data: Data { tasks: vec![] },
            error_text: None,
        }
    }

    fn title(&self) -> String {
        String::from("Organizer")
    }

    #[cfg(not(tarpaulin_include))]
    fn view(&self) -> Element<Message> {
        use iced::widget::Text;

        let data_view = self.data.view();
        let mut a_column = column(vec![]);
        if let Some(ref error_text) = self.error_text {
            a_column = a_column.push(Text::new(error_text));
        }
        a_column.push(data_view).into()
    }

    fn update(&mut self, message: Message) {
        self.error_text = None;
        match message {
            Message::AddTask => self.add_task(),
            Message::Task(task_id, task_message) => {
                if task_id > self.data.tasks.len() {
                    panic!("Tried to update inexisting task.")
                };
                self.process_task_message(task_id, task_message)
            }
            Message::Save => {
                let save_result = self.data.save("test");
                if let Err(error) = save_result {
                    self.error_text =
                        Some(format!("{0:?} problem: {1:?}", error.kind, error.message));
                }
            }
            Message::Load => {
                let loaded_data = Data::load("test");
                match loaded_data {
                    Ok(loaded_data) => self.data = loaded_data,
                    Err(error) => {
                        self.error_text =
                            Some(format!("{0:?} problem: {1:?}", error.kind, error.message))
                    }
                }
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

            organizer.update(Message::Task(1, task::Message::DeleteTask));
            assert_eq!(organizer.data.tasks.len(), 2);
        }

        #[test]
        fn task_message() {
            let mut organizer = Organizer::new();

            organizer.update(Message::AddTask);
            organizer.update(Message::AddTask);
            organizer.update(Message::AddTask);
            assert_eq!(organizer.data.tasks.len(), 3);

            organizer.update(Message::Task(0, task::Message::EditTask));
            organizer.update(Message::Task(1, task::Message::EditTask));
            organizer.update(Message::Task(2, task::Message::EditTask));

            organizer.data.tasks.iter().for_each(|task| {
                assert!(matches!(task.state(), task::State::BeingEdited));
            });

            organizer.update(Message::Task(0, task::Message::TextInput("A".to_string())));
            organizer.update(Message::Task(1, task::Message::TextInput("B".to_string())));
            organizer.update(Message::Task(2, task::Message::TextInput("C".to_string())));

            organizer.update(Message::Task(0, task::Message::FinishedEdition));
            organizer.update(Message::Task(1, task::Message::FinishedEdition));
            organizer.update(Message::Task(2, task::Message::FinishedEdition));

            organizer.data.tasks.iter().for_each(|task| {
                assert!(matches!(task.state(), task::State::Idle));
            });

            organizer.update(Message::Task(1, task::Message::DeleteTask));
            assert_eq!(organizer.data.tasks.len(), 2);

            assert_eq!(organizer.data.tasks[0].description(), "A");
            assert_eq!(organizer.data.tasks[1].description(), "C");
        }
    }
}
