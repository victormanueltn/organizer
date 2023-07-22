use crate::time::Duration;
use crate::Time;
use crate::ViewType;
use crate::{add_button, Organizer, SummaryDates};

#[derive(Debug, Clone)]
pub enum Message {
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

pub(crate) trait SummaryView {
    fn view_as_summary(&self) -> iced::Element<Message>;
    fn update_summary_view(&mut self, message: Message);
}

impl SummaryView for Organizer {
    fn view_as_summary(&self) -> iced::Element<Message> {
        let view_pick_list =
            iced::widget::pick_list(&ViewType::ALL[..], self.view_type, Message::SelectView);

        let pick_list_row = iced::widget::row!(view_pick_list).spacing(10).padding(10);

        let last_day_button = add_button("Last day", Message::LastDay);
        let last_week_button = add_button("Last week", Message::LastWeek);
        let last_two_weeks_button = add_button("Last two week", Message::LastTwoWeeks);
        let periods_row =
            iced::widget::row!(last_day_button, last_week_button, last_two_weeks_button)
                .spacing(10)
                .padding(10);

        let initial_day = self.summary_dates.initial_day.to_string();
        let initial_day_input =
            iced::widget::text_input("Initial day", &initial_day, Message::UpdateInitialDay)
                .padding(10);

        let initial_month = self.summary_dates.initial_month.to_string();
        let initial_month_input =
            iced::widget::text_input("Initial month", &initial_month, Message::UpdateInitialMonth)
                .padding(10);

        let initial_year = self.summary_dates.initial_year.to_string();
        let initial_year_input =
            iced::widget::text_input("Initial year", &initial_year, Message::UpdateInitialYear)
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
            iced::widget::text_input("Final day", &final_day, Message::UpdateFinalDay).padding(10);

        let final_month = self.summary_dates.final_month.to_string();
        let final_month_input =
            iced::widget::text_input("Initial month", &final_month, Message::UpdateFinalMonth)
                .padding(10);

        let final_year = self.summary_dates.final_year.to_string();
        let final_year_input =
            iced::widget::text_input("Initial year", &final_year, Message::UpdateFinalYear)
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

    fn update_summary_view(&mut self, message: Message) {
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
            Message::SelectView(value) => self.view_type = Some(value),
            Message::UpdateInitialDay(value) => {
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
            Message::UpdateInitialMonth(value) => {
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
            Message::UpdateInitialYear(value) => {
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
            Message::UpdateFinalDay(value) => {
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
            Message::UpdateFinalMonth(value) => {
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
            Message::UpdateFinalYear(value) => {
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
            Message::LastDay => {
                let now = Time::now();
                let before = &now - &Duration::from_hours(24);
                self.summary_dates = SummaryDates::new(&before, &now);
            }
            Message::LastWeek => {
                let now = Time::now();
                let before = &now - &Duration::from_hours(24 * 7);
                self.summary_dates = SummaryDates::new(&before, &now);
            }
            Message::LastTwoWeeks => {
                let now = Time::now();
                let before = &now - &Duration::from_hours(24 * 14);
                self.summary_dates = SummaryDates::new(&before, &now);
            }
        }
    }
}
