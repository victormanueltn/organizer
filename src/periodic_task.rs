use crate::add_button;
use crate::Time;
use crate::TimeError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct PeriodicTask {
    description: String,
    frequency: Option<usize>,
    time_period: Option<TimePeriod>,
    initial_day: u32,
    initial_month: u32,
    initial_year: u32,
    initial_date: Result<Time, TimeError>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
enum TimePeriod {
    Day,
    Month,
    Year,
}

impl PeriodicTask {
    pub(crate) fn new(description: String) -> Self {
        let now = Time::now();
        PeriodicTask {
            description,
            frequency: None,
            time_period: None,
            initial_day: now.day(),
            initial_month: now.month(),
            initial_year: now.year(),
            initial_date: Time::new(now.day(), now.month(), now.year(), 0, 0, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TextInput(String),
    DeleteTask,
    UpdateInitialDay(String),
    UpdateInitialMonth(String),
    UpdateInitialYear(String),
}

pub(crate) trait ToIced {
    type Message;
    fn view(&self) -> iced::Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
}

impl ToIced for PeriodicTask {
    type Message = Message;
    fn view(&self) -> iced::Element<Self::Message> {
        let text_input = iced::widget::text_input(
            "Describe your task...",
            &self.description,
            Self::Message::TextInput,
        );

        let delete_button =
            add_button("Delete", Message::DeleteTask).style(iced::theme::Button::Destructive);

        let description_row = iced::widget::row(vec![])
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(text_input)
            .push(delete_button);

        let initial_day = self.initial_day.to_string();
        let initial_day_input =
            iced::widget::text_input("Initial day", &initial_day, Message::UpdateInitialDay)
                .padding(10);

        let initial_month = self.initial_month.to_string();
        let initial_month_input =
            iced::widget::text_input("Initial month", &initial_month, Message::UpdateInitialMonth)
                .padding(10);

        let initial_year = self.initial_year.to_string();
        let initial_year_input =
            iced::widget::text_input("Initial year", &initial_year, Message::UpdateInitialYear)
                .padding(10);
        let initial_date_row =
            iced::widget::row![initial_day_input, initial_month_input, initial_year_input];

        let initial_date_label =
            iced::widget::row![iced::widget::text("Initial date: Day/Month/Year")];

        let initial_date = Time::new(
            self.initial_day,
            self.initial_month,
            self.initial_year,
            0,
            0,
            0,
        );

        let column = {
            let mut column =
                iced::widget::column![initial_date_label, initial_date_row, description_row];

            if initial_date.is_err() {
                column = column.push(iced::widget::row![iced::widget::text(
                    "WRONG INITIAL DATE: date does not exist!"
                )]);
            }
            column
        };

        column.into()
    }

    fn update(&mut self, message: Self::Message) {
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
            Message::TextInput(description) => self.description = description,
            Message::DeleteTask => panic!(), // Delete task is not used by PeriodicTask.
            Message::UpdateInitialDay(value) => {
                handle_update(&value, 31, &mut self.initial_day);
                self.initial_date = Time::new(
                    self.initial_day,
                    self.initial_month,
                    self.initial_year,
                    0,
                    0,
                    0,
                );
            }
            Message::UpdateInitialMonth(value) => {
                handle_update(&value, 12, &mut self.initial_month);
                self.initial_date = Time::new(
                    self.initial_day,
                    self.initial_month,
                    self.initial_year,
                    0,
                    0,
                    0,
                );
            }
            Message::UpdateInitialYear(value) => {
                handle_update(&value, 10000, &mut self.initial_year);
                self.initial_date = Time::new(
                    self.initial_day,
                    self.initial_month,
                    self.initial_year,
                    0,
                    0,
                    0,
                );
            }
        };
    }
}
