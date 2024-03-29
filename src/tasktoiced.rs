use crate::task::{self, Task};
use crate::time::Duration;
use crate::time::Time;
use crate::toiced::add_button;
use crate::toiced::ToIced;
use iced::widget::text_input::StyleSheet;
use iced::widget::{checkbox, column, row, text_input};
use iced::Element;

struct TextInputStyle {
    theme: iced::theme::Theme,
    text_transparency: f32,
}

pub(crate) const FADE_OUT_TIME: i64 = 60 * 24;

impl StyleSheet for TextInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        let palette = self.theme.extended_palette();

        let mut test_color = palette.background.base.text;
        test_color.a = 0.1;

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: test_color,
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        let palette = self.theme.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.background.strong.text,
        }
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        let palette = self.theme.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.background.base.text,
        }
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = self.theme.extended_palette();

        let mut color = palette.background.base.text;
        color.a = self.text_transparency;
        color
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = self.theme.extended_palette();
        palette.primary.weak.color
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = self.theme.extended_palette();
        palette.background.strong.color
    }
}

impl ToIced for Task {
    type Message = task::Message;
    fn view(&self) -> Element<task::Message> {
        let a_checkbox = checkbox(
            "".to_string(),
            self.completed(),
            task::Message::ToggleTaskCompletion,
        );

        let text_transparency = {
            if !self.completed() {
                1.
            } else if let Some(ref completion_time) = self.completion_time {
                let elapsed_time = &Time::now() - completion_time;
                let fade_out_time = Duration::from_minutes(FADE_OUT_TIME);
                if elapsed_time < fade_out_time {
                    1. - elapsed_time / fade_out_time
                } else {
                    1.
                }
            } else {
                1.
            }
        };

        let text_input_style = TextInputStyle {
            theme: iced::Theme::Light,
            text_transparency,
        };

        let text_input_theme = iced::theme::TextInput::Custom(Box::new(text_input_style));

        let a_text_input = text_input(
            "Describe your task...",
            self.description(),
            task::Message::TextInput,
        )
        .padding(10)
        .style(text_input_theme);

        let delete_button =
            add_button("Delete", task::Message::DeleteTask).style(iced::theme::Button::Destructive);

        let a_row = {
            let mut a_row = row(vec![])
                .spacing(10)
                .padding(10)
                .align_items(iced::Alignment::Center)
                .push(a_checkbox)
                .push(a_text_input)
                .push(delete_button);
            if self.hidden_because_of_snooze() {
                let unsnooze_button = add_button("Unsnooze", task::Message::Unsnooze)
                    .style(iced::theme::Button::Secondary);
                a_row = a_row.push(unsnooze_button);
            } else {
                let snooze_button = add_button("Snooze", task::Message::AddSnoozeTime)
                    .style(iced::theme::Button::Secondary);
                a_row = a_row.push(snooze_button);
            }
            a_row
        };

        let mut a_column = column(vec![]).push(a_row);

        let snooze_duration_row = if self.snooze_information.visible {
            let quantity = self.snooze_information.quantity.to_string();
            let quantity_input =
                iced::widget::text_input(&quantity, &quantity, task::Message::SetSnoozeQuantity)
                    .padding(10);
            let hour = add_button(
                "Hour",
                task::Message::SetSnoozeDuration(task::SnoozeDuration::Hour),
            )
            .style(iced::theme::Button::Secondary);
            let day = add_button(
                "Day",
                task::Message::SetSnoozeDuration(task::SnoozeDuration::Day),
            )
            .style(iced::theme::Button::Secondary);
            let week = add_button(
                "Week",
                task::Message::SetSnoozeDuration(task::SnoozeDuration::Week),
            )
            .style(iced::theme::Button::Secondary);
            let month = add_button(
                "Month",
                task::Message::SetSnoozeDuration(task::SnoozeDuration::Month),
            )
            .style(iced::theme::Button::Secondary);

            Some(
                row(vec![])
                    .push(quantity_input)
                    .push(hour)
                    .push(day)
                    .push(week)
                    .push(month),
            )
        } else {
            None
        };

        if let Some(snooze_duration_row) = snooze_duration_row {
            a_column = a_column.push(snooze_duration_row);
        }

        a_column.into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            task::Message::ToggleTaskCompletion(completed) => {
                self.set_completed(completed);
                if !completed {
                    self.completion_time = None
                }
            }
            task::Message::TextInput(description) => self.edit(&description),
            task::Message::DeleteTask => {
                unreachable!();
            }
            task::Message::AddSnoozeTime => {
                self.snooze_information.visible = true;
                self.snooze_information.snooze_until = Some(Time::now());
            }
            task::Message::SetSnoozeQuantity(value) => {
                let value = {
                    let mut result: u32 = 1;
                    if value.is_empty() {
                        result = 1;
                    } else if let Ok(value) = value.parse::<u32>() {
                        result = value
                    }
                    result
                };
                self.snooze_information.quantity = value;
            }
            task::Message::Unsnooze => {
                self.snooze_information.visible = false;
                self.snooze_information.snooze_until = None;
            }
            task::Message::SetSnoozeDuration(duration) => {
                self.snooze_information.visible = false;
                let quantity = self.snooze_information.quantity;
                let duration: Duration = match duration {
                    task::SnoozeDuration::Hour => Duration::from_hours(quantity as i64 * 1),
                    task::SnoozeDuration::Day => Duration::from_hours(quantity as i64 * 24),
                    task::SnoozeDuration::Week => Duration::from_hours(quantity as i64 * 24 * 7),
                    task::SnoozeDuration::Month => Duration::from_hours(quantity as i64 * 24 * 30),
                };
                self.snooze_information.snooze_until =
                    Some(self.snooze_information.snooze_until.as_ref().unwrap() + &duration);
            }
        }
    }
}
