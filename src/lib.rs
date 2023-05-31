mod data;
mod task;
mod tasktoiced;
mod views;
use crate::views::{ListMessage, Message, ViewType};
use data::{Data, Filters};
use views::SummaryMessage;
mod datatoiced;
mod time;
mod toiced;
use crate::toiced::add_button;
use iced::widget::pick_list;
use iced::widget::text_input;
use iced::widget::{column, row};
use iced::Element;
use iced::Sandbox;
use iced::{widget::Text, Alignment};
use task::Task;
use toiced::ToIced;

pub struct Organizer {
    data: Data,
    error_text: Option<String>,
    file_name: String,
    view_type: Option<ViewType>,
    summary_dates: SummaryDates,
}

struct SummaryDates {
    initial_day: u32,
    initial_month: u32,
    initial_year: u32,
}

#[cfg(not(tarpaulin_include))]
impl Sandbox for Organizer {
    type Message = Message;

    fn new() -> Self {
        Organizer {
            data: Data {
                tasks: vec![],
                filters: Filters {
                    todo: true,
                    complete: false,
                },
            },
            error_text: None,
            file_name: String::new(),
            view_type: Some(ViewType::List),
            summary_dates: SummaryDates {
                initial_day: 0,
                initial_month: 0,
                initial_year: 2023,
            },
        }
    }

    fn title(&self) -> String {
        String::from("Organizer")
    }

    fn view(&self) -> Element<Message> {
        match self.view_type.unwrap() {
            ViewType::List => self.view_as_list().map(Message::ListViewMessage),
            ViewType::Summary => self.view_as_summary().map(Message::SummaryViewMessage),
        }
    }

    fn update(&mut self, message: Message) {
        self.error_text = None;
        match message {
            Message::ListViewMessage(message) => self.update_list_view(message),
            Message::SummaryViewMessage(message) => self.update_summary_view(message),
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
                task::Message::TextInput(description) => a_task.edit(&description),
                task::Message::DeleteTask => {
                    self.data.tasks.remove(task_id);
                }
            }
        }
    }

    fn view_as_list(&self) -> Element<ListMessage> {
        let button_todo_tasks = iced::widget::Checkbox::new(
            "Todo",
            self.data.filters.todo,
            ListMessage::ToggleActiveFilter,
        );
        let button_complete_tasks = iced::widget::Checkbox::new(
            "Complete",
            self.data.filters.complete,
            ListMessage::ToggleCompleteFilter,
        );

        let a_row = row![button_todo_tasks, button_complete_tasks].spacing(40);

        let data_view = self.data.view();
        let mut a_column = column(vec![a_row.into()]).align_items(Alignment::Center);
        if let Some(ref error_text) = self.error_text {
            a_column = a_column
                .push(Text::new(error_text).style(iced::Color::from_rgb(1., 0., 0.)))
                .align_items(Alignment::Center);
        }

        let file_name_input = text_input(
            "Name of the task list",
            &self.file_name,
            ListMessage::UpdateSaveFileName,
        )
        .padding(10);
        let load_button = add_button("Save task list", ListMessage::Save);
        let save_button = add_button("Load task list", ListMessage::Load);
        let a_row = row!(file_name_input, save_button, load_button)
            .spacing(10)
            .padding(10);

        let view_pick_list = pick_list(&ViewType::ALL[..], self.view_type, ListMessage::SelectView);

        a_column
            .push(a_row)
            .push(data_view)
            .push(view_pick_list)
            .spacing(10)
            .into()
    }

    fn view_as_summary(&self) -> Element<SummaryMessage> {
        let view_pick_list = pick_list(
            &ViewType::ALL[..],
            self.view_type,
            SummaryMessage::SelectView,
        );

        let a_row = row!(view_pick_list).spacing(10).padding(10);

        let initial_day = self.summary_dates.initial_day.to_string();
        let initial_day_input =
            text_input("Initial", &initial_day, SummaryMessage::UpdateInitialDay).padding(10);

        let initial_month = self.summary_dates.initial_month.to_string();
        let initial_month_input = text_input(
            "Initial",
            &initial_month,
            SummaryMessage::UpdateInitialMonth,
        )
        .padding(10);

        let initial_year = self.summary_dates.initial_year.to_string();
        let initial_year_input =
            text_input("Initial", &initial_year, SummaryMessage::UpdateInitialYear).padding(10);
        let initial_date_row = row![initial_day_input, initial_month_input, initial_year_input];

        let a_column = column(vec![])
            .push(a_row)
            .push(initial_date_row)
            .spacing(10)
            .align_items(Alignment::Center)
            .into();
        a_column
    }

    fn update_summary_view(&mut self, message: SummaryMessage) {
        let handle_update = |value: &str, max_value: u32, result: &mut u32| {
            if value.is_empty() {
                *result = 0;
            } else if let Ok(day) = value.parse::<u32>() {
                if day <= max_value {
                    *result = day
                }
            }
        };
        match message {
            SummaryMessage::SelectView(value) => self.view_type = Some(value),
            SummaryMessage::UpdateInitialDay(value) => {
                handle_update(&value, 31, &mut self.summary_dates.initial_day);
            }
            SummaryMessage::UpdateInitialMonth(value) => {
                handle_update(&value, 12, &mut self.summary_dates.initial_month);
            }
            SummaryMessage::UpdateInitialYear(value) => {
                handle_update(&value, 10000, &mut self.summary_dates.initial_year);
            }
        }
    }

    fn update_list_view(&mut self, message: ListMessage) {
        match message {
            ListMessage::AddTask => self.add_task(),
            ListMessage::Task(task_id, task_message) => {
                if task_id > self.data.tasks.len() {
                    panic!("Tried to update inexisting task.")
                };
                self.process_task_message(task_id, task_message)
            }
            ListMessage::UpdateSaveFileName(file_name) => {
                self.file_name = file_name;
            }
            ListMessage::Load => {
                let loaded_data = Data::load(&self.file_name);
                match loaded_data {
                    Ok(loaded_data) => self.data = loaded_data,
                    Err(error) => {
                        self.error_text =
                            Some(format!("{0:?} problem: {1:?}", error.kind, error.message))
                    }
                }
            }
            ListMessage::Save => {
                let save_result = self.data.save(&self.file_name);
                if let Err(error) = save_result {
                    self.error_text =
                        Some(format!("{0:?} problem: {1:?}", error.kind, error.message));
                }
            }
            ListMessage::ToggleActiveFilter(value) => {
                self.data.filters.todo = value;
            }
            ListMessage::ToggleCompleteFilter(value) => {
                self.data.filters.complete = value;
            }
            ListMessage::SelectView(value) => self.view_type = Some(value),
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
            organizer.update(Message::ListViewMessage(ListMessage::AddTask));
            assert_eq!(organizer.data.tasks.len(), 1);
        }

        #[test]
        #[should_panic]
        fn message_to_inexisting_task() {
            let mut organizer = Organizer::new();
            organizer.update(Message::ListViewMessage(ListMessage::AddTask));

            organizer.update(Message::ListViewMessage(ListMessage::Task(
                1,
                task::Message::DeleteTask,
            )));
            assert_eq!(organizer.data.tasks.len(), 2);
        }

        #[test]
        fn task_message() {
            let mut organizer = Organizer::new();

            organizer.update(Message::ListViewMessage(ListMessage::AddTask));
            organizer.update(Message::ListViewMessage(ListMessage::AddTask));
            organizer.update(Message::ListViewMessage(ListMessage::AddTask));
            assert_eq!(organizer.data.tasks.len(), 3);

            organizer.update(Message::ListViewMessage(ListMessage::Task(
                0,
                task::Message::TextInput("A".to_string()),
            )));
            organizer.update(Message::ListViewMessage(ListMessage::Task(
                1,
                task::Message::TextInput("B".to_string()),
            )));
            organizer.update(Message::ListViewMessage(ListMessage::Task(
                2,
                task::Message::TextInput("C".to_string()),
            )));

            organizer.update(Message::ListViewMessage(ListMessage::Task(
                1,
                task::Message::DeleteTask,
            )));
            assert_eq!(organizer.data.tasks.len(), 2);

            assert_eq!(organizer.data.tasks[0].description(), "A");
            assert_eq!(organizer.data.tasks[1].description(), "C");
        }
    }
}
