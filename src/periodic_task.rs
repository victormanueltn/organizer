use crate::add_button;
use crate::task::Task;
use crate::Duration;
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
    last_created: Time,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
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
            last_created: now,
        }
    }

    pub(crate) fn create_tasks(&mut self) -> Vec<Task> {
        if self.description.is_empty() {
            return vec![];
        }
        let required = (&self.frequency, &self.time_period);
        if let (Some(ref _frequency), Some(ref _time_period)) = required {
            let now = Time::now();
            let period = Duration::from_hours(i64::try_from(self.period_in_hours()).unwrap());
            let mut tasks = vec![];
            while &self.last_created + &period < now {
                let previous = self.last_created.clone();
                self.last_created = &previous + &period;
                let description = self.description.clone() + " - "+ &self.last_created.to_string();
                let mut task = Task::new(0);
                task.edit(&description);
                tasks.push(task);
            }
            tasks
        } else {
            vec![]
        }
    }

    fn period_in_hours(&self) -> usize {
        const HOURS_PER_DAY: usize = 24;
        const DAYS_PER_WEEK: usize = 7;
        const WEEKS_PER_MONTH: usize = 4;
        const MONTHS_PER_YEAR: usize = 12;
        let frequency = self.frequency.unwrap();
        match self.time_period.as_ref().unwrap() {
            TimePeriod::Daily => HOURS_PER_DAY / frequency,
            TimePeriod::Weekly => DAYS_PER_WEEK * HOURS_PER_DAY / frequency,
            TimePeriod::Monthly => WEEKS_PER_MONTH * DAYS_PER_WEEK * HOURS_PER_DAY / frequency,
            TimePeriod::Yearly => {
                MONTHS_PER_YEAR * WEEKS_PER_MONTH * DAYS_PER_WEEK * HOURS_PER_DAY / frequency
            }
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
    Daily,
    Weekly,
    Monthly,
    Yearly,
    UpdateFrequency(String),
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

        let frequency_row = {
            let frequency = self.frequency.unwrap_or(0usize).to_string();
            let frequency_input =
                iced::widget::text_input("Frequency", &frequency, Message::UpdateFrequency)
                    .padding(10);
            let daily = add_button("Daily", Message::Daily);
            let weekly = add_button("Weekly", Message::Weekly);
            let monthly = add_button("Monthly", Message::Monthly);
            let yearly = add_button("Yearly", Message::Yearly);
            iced::widget::row!(frequency_input, daily, weekly, monthly, yearly)
                .spacing(10)
                .padding(10)
        };

        let delete_button =
            add_button("Delete", Message::DeleteTask).style(iced::theme::Button::Destructive);

        let description_row = iced::widget::row(vec![])
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(text_input)
            .push(delete_button);

        let initial_date_row = {
            let initial_day = self.initial_day.to_string();
            let initial_day_input =
                iced::widget::text_input("Initial day", &initial_day, Message::UpdateInitialDay)
                    .padding(10);

            let initial_month = self.initial_month.to_string();
            let initial_month_input = iced::widget::text_input(
                "Initial month",
                &initial_month,
                Message::UpdateInitialMonth,
            )
            .padding(10);

            let initial_year = self.initial_year.to_string();
            let initial_year_input =
                iced::widget::text_input("Initial year", &initial_year, Message::UpdateInitialYear)
                    .padding(10);

            iced::widget::row![initial_day_input, initial_month_input, initial_year_input]
        };

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
            let mut column = iced::widget::column![
                frequency_row,
                initial_date_label,
                initial_date_row,
                description_row
            ];

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
            Message::Daily => self.time_period = Some(TimePeriod::Daily),
            Message::Weekly => self.time_period = Some(TimePeriod::Weekly),
            Message::Monthly => self.time_period = Some(TimePeriod::Monthly),
            Message::Yearly => self.time_period = Some(TimePeriod::Monthly),
            Message::UpdateFrequency(frequency) => {
                self.frequency = {
                    if frequency.is_empty() {
                        None
                    } else if let Ok(frequency) = frequency.parse::<usize>() {
                        Some(frequency)
                    } else {
                        None
                    }
                };
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn period_in_hours() {
        let mut periodic_task = PeriodicTask::new("test".to_string());

        periodic_task.frequency = Some(1);
        periodic_task.time_period = Some(TimePeriod::Daily);
        assert_eq!(periodic_task.period_in_hours(), 24);

        periodic_task.frequency = Some(1);
        periodic_task.time_period = Some(TimePeriod::Weekly);
        assert_eq!(periodic_task.period_in_hours(), 24 * 7);

        periodic_task.frequency = Some(1);
        periodic_task.time_period = Some(TimePeriod::Monthly);
        assert_eq!(periodic_task.period_in_hours(), 24 * 7 * 4);

        periodic_task.frequency = Some(1);
        periodic_task.time_period = Some(TimePeriod::Yearly);
        assert_eq!(periodic_task.period_in_hours(), 24 * 7 * 4 * 12);

        periodic_task.frequency = Some(2);
        periodic_task.time_period = Some(TimePeriod::Daily);
        assert_eq!(periodic_task.period_in_hours(), 24 / 2);

        periodic_task.frequency = Some(2);
        periodic_task.time_period = Some(TimePeriod::Weekly);
        assert_eq!(periodic_task.period_in_hours(), 24 / 2 * 7);

        periodic_task.frequency = Some(2);
        periodic_task.time_period = Some(TimePeriod::Monthly);
        assert_eq!(periodic_task.period_in_hours(), 24 / 2 * 7 * 4);

        periodic_task.frequency = Some(2);
        periodic_task.time_period = Some(TimePeriod::Yearly);
        assert_eq!(periodic_task.period_in_hours(), 24 / 2 * 7 * 4 * 12);
    }
}
