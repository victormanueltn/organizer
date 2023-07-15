use crate::time::Duration;
use crate::toiced::ToIced;
use crate::{add_button, task, Organizer, SummaryDates};
use crate::{Data, Text, Time};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    List,
    Summary,
}

impl ViewType {
    pub const ALL: [ViewType; 2] = [ViewType::List, ViewType::Summary];
}

impl std::fmt::Display for ViewType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ViewType::List => "List",
                ViewType::Summary => "Summary",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ListViewMessage(ListMessage),
    SummaryViewMessage(SummaryMessage),
}

#[derive(Debug, Clone)]
pub enum ListMessage {
    AddTask,
    Task(usize, task::Message),
    Load,
    UpdateSaveFileName(String),
    Save,
    ToggleActiveFilter(bool),
    ToggleCompleteFilter(bool),
    SwapWithPrevious(usize),
    SwapWithNext(usize),
    SelectView(ViewType),
}

#[derive(Debug, Clone)]
pub enum SummaryMessage {
    SelectView(ViewType),
    UpdateInitialDay(String),
    UpdateInitialMonth(String),
    UpdateInitialYear(String),
    UpdateFinalDay(String),
    UpdateFinalMonth(String),
    UpdateFinalYear(String),
    LastDay,
    LastWeek,
    LastTwoWeeks,
}

pub(crate) trait ListView {
    fn view_as_list(&self) -> iced::Element<ListMessage>;
    fn update_list_view(&mut self, message: ListMessage);
}

impl ListView for Organizer {
    fn view_as_list(&self) -> iced::Element<ListMessage> {
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

        let a_row = iced::widget::row![button_todo_tasks, button_complete_tasks].spacing(40);

        let data_view = self.data.view();
        let mut a_column =
            iced::widget::column(vec![a_row.into()]).align_items(iced::Alignment::Center);
        if let Some(ref error_text) = self.error_text {
            a_column = a_column
                .push(Text::new(error_text).style(iced::Color::from_rgb(1., 0., 0.)))
                .align_items(iced::Alignment::Center);
        }

        let file_name = &self.file_name.clone().unwrap_or(String::new());
        let file_name_input = iced::widget::text_input(
            "Name of the task list",
            file_name,
            ListMessage::UpdateSaveFileName,
        )
        .padding(10);
        let load_button = add_button("Save task list", ListMessage::Save);
        let save_button = add_button("Load task list", ListMessage::Load);
        let a_row = iced::widget::row!(file_name_input, save_button, load_button)
            .spacing(10)
            .padding(10);

        let view_pick_list =
            iced::widget::pick_list(&ViewType::ALL[..], self.view_type, ListMessage::SelectView);

        a_column
            .push(a_row)
            .push(data_view)
            .push(view_pick_list)
            .spacing(10)
            .into()
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
                self.file_name = Some(file_name);
            }
            ListMessage::Load => {
                let loaded_data = Data::load(&self.file_name.clone().unwrap_or(String::new()));
                match loaded_data {
                    Ok(loaded_data) => self.data = loaded_data,
                    Err(error) => {
                        self.error_text =
                            Some(format!("{0:?} problem: {1:?}", error.kind, error.message))
                    }
                }
            }
            ListMessage::Save => {
                let save_result = self.data.save(self.file_name.as_ref().unwrap());
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
            ListMessage::SwapWithPrevious(index) => {
                let first_visible = index == 0;
                if !first_visible {
                    let visible_tasks = self.data.visible_tasks();
                    let (current_index, _) = visible_tasks[index];
                    let (previous_index, _) = visible_tasks[index - 1];
                    self.data.tasks.swap(current_index, previous_index);
                }
            }
            ListMessage::SwapWithNext(index) => {
                let visible_tasks = self.data.visible_tasks();
                let last_visible = index + 1 == visible_tasks.len();
                if !last_visible {
                    let (current_index, _) = visible_tasks[index];
                    let (next_index, _) = visible_tasks[index + 1];
                    self.data.tasks.swap(current_index, next_index);
                }
            }

            ListMessage::SelectView(value) => self.view_type = Some(value),
        }
    }
}

pub(crate) trait SummaryView {
    fn view_as_summary(&self) -> iced::Element<SummaryMessage>;
    fn update_summary_view(&mut self, message: SummaryMessage);
}

impl SummaryView for Organizer {
    fn view_as_summary(&self) -> iced::Element<SummaryMessage> {
        let view_pick_list = iced::widget::pick_list(
            &ViewType::ALL[..],
            self.view_type,
            SummaryMessage::SelectView,
        );

        let pick_list_row = iced::widget::row!(view_pick_list).spacing(10).padding(10);

        let last_day_button = add_button("Last day", SummaryMessage::LastDay);
        let last_week_button = add_button("Last week", SummaryMessage::LastWeek);
        let last_two_weeks_button = add_button("Last two week", SummaryMessage::LastTwoWeeks);
        let periods_row =
            iced::widget::row!(last_day_button, last_week_button, last_two_weeks_button)
                .spacing(10)
                .padding(10);

        let initial_day = self.summary_dates.initial_day.to_string();
        let initial_day_input = iced::widget::text_input(
            "Initial day",
            &initial_day,
            SummaryMessage::UpdateInitialDay,
        )
        .padding(10);

        let initial_month = self.summary_dates.initial_month.to_string();
        let initial_month_input = iced::widget::text_input(
            "Initial month",
            &initial_month,
            SummaryMessage::UpdateInitialMonth,
        )
        .padding(10);

        let initial_year = self.summary_dates.initial_year.to_string();
        let initial_year_input = iced::widget::text_input(
            "Initial year",
            &initial_year,
            SummaryMessage::UpdateInitialYear,
        )
        .padding(10);
        let initial_date_row =
            iced::widget::row![initial_day_input, initial_month_input, initial_year_input];

        let initial_date_label =
            iced::widget::row![iced::widget::text("Initial date: Day/Month/Year")];

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
            iced::widget::text_input("Final day", &final_day, SummaryMessage::UpdateFinalDay)
                .padding(10);

        let final_month = self.summary_dates.final_month.to_string();
        let final_month_input = iced::widget::text_input(
            "Initial month",
            &final_month,
            SummaryMessage::UpdateFinalMonth,
        )
        .padding(10);

        let final_year = self.summary_dates.final_year.to_string();
        let final_year_input =
            iced::widget::text_input("Initial year", &final_year, SummaryMessage::UpdateFinalYear)
                .padding(10);
        let final_date_row =
            iced::widget::row![final_day_input, final_month_input, final_year_input];

        let final_date_label = iced::widget::row![iced::widget::text("Final date: Day/Month/Year")];

        let final_date = Time::new(
            self.summary_dates.final_day,
            self.summary_dates.final_month,
            self.summary_dates.final_year,
            23,
            59,
            59,
        );

        let mut a_column = iced::widget::column(vec![]);

        let descriptions = if let (Ok(initial_date), Ok(final_date)) = (&initial_date, &final_date)
        {
            self.data
                .tasks
                .iter()
                .filter(|task| task.completed())
                .filter(|task| task.completion_time.is_some())
                .filter(|task| {
                    let completion_time = &task.completion_time.clone().unwrap();
                    initial_date < completion_time && completion_time < final_date
                })
                .map(|task| iced::widget::row![iced::widget::text(task.description())])
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        a_column = a_column
            .push(pick_list_row)
            .push(periods_row)
            .push(initial_date_row)
            .push(initial_date_label);

        if initial_date.is_err() {
            a_column = a_column.push(iced::widget::row![iced::widget::text(
                "WRONG INITIAL DATE: date does not exist!"
            )]);
        }

        a_column = a_column.push(final_date_row).push(final_date_label);

        if final_date.is_err() {
            a_column = a_column.push(iced::widget::row![iced::widget::text(
                "WRONG FINAL DATE: date does not exist!"
            )]);
        }

        for description in descriptions {
            a_column = a_column.push(description);
        }

        a_column
            .spacing(10)
            .align_items(iced::Alignment::Center)
            .into()
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
            SummaryMessage::LastDay => {
                let now = Time::now();
                let before = &now - &Duration::from_hours(24);
                self.summary_dates = SummaryDates::new(&before, &now);
            }
            SummaryMessage::LastWeek => {
                let now = Time::now();
                let before = &now - &Duration::from_hours(24 * 7);
                self.summary_dates = SummaryDates::new(&before, &now);
            }
            SummaryMessage::LastTwoWeeks => {
                let now = Time::now();
                let before = &now - &Duration::from_hours(24 * 14);
                self.summary_dates = SummaryDates::new(&before, &now);
            }
        }
    }
}
