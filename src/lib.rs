mod data;
mod datatoiced;
mod task;
mod tasktoiced;
mod time;
mod toiced;
mod views;
use std::vec;

use crate::toiced::add_button;
use crate::views::{ListMessage, Message, ViewType};
use data::{Data, Filters};
use iced::widget::pick_list;
use iced::widget::text;
use iced::widget::text_input;
use iced::widget::{column, row};
use iced::Element;
use iced::Sandbox;
use iced::{widget::Text, Alignment};
use task::Task;
use time::{Time, TimeError};
use toiced::ToIced;
use views::SummaryMessage;

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
    initial_date: Result<Time, TimeError>,
    final_day: u32,
    final_month: u32,
    final_year: u32,
    final_date: Result<Time, TimeError>,
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
                initial_day: 1,
                initial_month: 1,
                initial_year: 2023,
                initial_date: Time::new(1, 1, 2023, 0, 0, 0),
                final_day: 31,
                final_month: 12,
                final_year: 3023,
                final_date: Time::new(31, 12, 3023, 0, 0, 0),
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
        let initial_day_input = text_input(
            "Initial day",
            &initial_day,
            SummaryMessage::UpdateInitialDay,
        )
        .padding(10);

        let initial_month = self.summary_dates.initial_month.to_string();
        let initial_month_input = text_input(
            "Initial month",
            &initial_month,
            SummaryMessage::UpdateInitialMonth,
        )
        .padding(10);

        let initial_year = self.summary_dates.initial_year.to_string();
        let initial_year_input = text_input(
            "Initial year",
            &initial_year,
            SummaryMessage::UpdateInitialYear,
        )
        .padding(10);
        let initial_date_row = row![initial_day_input, initial_month_input, initial_year_input];

        let initial_date_label = row![text("Initial date: Day/Month/Year")];

        let initial_date = Time::new(
            self.summary_dates.initial_day,
            self.summary_dates.initial_month,
            self.summary_dates.initial_year,
            0,
            0,
            0,
        );

        let final_day = self.summary_dates.final_day.to_string();
        let final_day_input =
            text_input("Final day", &final_day, SummaryMessage::UpdateFinalDay).padding(10);

        let final_month = self.summary_dates.final_month.to_string();
        let final_month_input = text_input(
            "Initial month",
            &final_month,
            SummaryMessage::UpdateFinalMonth,
        )
        .padding(10);

        let final_year = self.summary_dates.final_year.to_string();
        let final_year_input =
            text_input("Initial year", &final_year, SummaryMessage::UpdateFinalYear).padding(10);
        let final_date_row = row![final_day_input, final_month_input, final_year_input];

        let final_date_label = row![text("Final date: Day/Month/Year")];

        let final_date = Time::new(
            self.summary_dates.final_day,
            self.summary_dates.final_month,
            self.summary_dates.final_year,
            23,
            59,
            59,
        );

        let mut a_column = column(vec![]);

        let descriptions = if let (Ok(initial_date), Ok(final_date)) = (&initial_date, &final_date)
        {
            self.data
                .tasks
                .iter()
                .filter(|task| task.completion_time.is_some())
                .filter(|task| {
                    let completion_time = &task.completion_time.clone().unwrap();
                    initial_date < completion_time && completion_time < final_date
                })
                .map(|task| row![text(task.description())])
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        a_column = a_column
            .push(a_row)
            .push(initial_date_row)
            .push(initial_date_label);

        if let Err(_) = initial_date {
            a_column = a_column.push(row![text("WRONG INITIAL DATE: date does not exist!")]);
        }

        a_column = a_column.push(final_date_row).push(final_date_label);

        if let Err(_) = final_date {
            a_column = a_column.push(row![text("WRONG FINAL DATE: date does not exist!")]);
        }

        for description in descriptions {
            a_column = a_column.push(description);
        }

        a_column.spacing(10).align_items(Alignment::Center).into()
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
                self.summary_dates.initial_date = Time::new(
                    self.summary_dates.initial_day,
                    self.summary_dates.initial_month,
                    self.summary_dates.initial_year,
                    0,
                    0,
                    0,
                );
            }
            SummaryMessage::UpdateInitialMonth(value) => {
                handle_update(&value, 12, &mut self.summary_dates.initial_month);
                self.summary_dates.initial_date = Time::new(
                    self.summary_dates.initial_day,
                    self.summary_dates.initial_month,
                    self.summary_dates.initial_year,
                    0,
                    0,
                    0,
                );
            }
            SummaryMessage::UpdateInitialYear(value) => {
                handle_update(&value, 10000, &mut self.summary_dates.initial_year);
                self.summary_dates.initial_date = Time::new(
                    self.summary_dates.initial_day,
                    self.summary_dates.initial_month,
                    self.summary_dates.initial_year,
                    0,
                    0,
                    0,
                );
            }
            SummaryMessage::UpdateFinalDay(value) => {
                handle_update(&value, 31, &mut self.summary_dates.final_day);
                self.summary_dates.final_date = Time::new(
                    self.summary_dates.final_day,
                    self.summary_dates.final_month,
                    self.summary_dates.final_year,
                    23,
                    59,
                    59,
                );
            }
            SummaryMessage::UpdateFinalMonth(value) => {
                handle_update(&value, 12, &mut self.summary_dates.final_month);
                self.summary_dates.final_date = Time::new(
                    self.summary_dates.final_day,
                    self.summary_dates.final_month,
                    self.summary_dates.final_year,
                    23,
                    59,
                    59,
                );
            }
            SummaryMessage::UpdateFinalYear(value) => {
                handle_update(&value, 10000, &mut self.summary_dates.final_year);
                self.summary_dates.final_date = Time::new(
                    self.summary_dates.final_day,
                    self.summary_dates.final_month,
                    self.summary_dates.final_year,
                    23,
                    59,
                    59,
                );
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
