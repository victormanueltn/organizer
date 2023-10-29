use crate::task::{self, Task};
use crate::time::Duration;
use crate::time::Time;
use crate::toiced::add_button;
use iced::widget::text_input::StyleSheet;
use iced::widget::{checkbox, row, text_input};
use iced::Element;

struct TextInputStyle {
    theme: iced::theme::Theme,
    text_transparency: f32,
}

pub(crate) trait TaskToIced {
    type Message;
    fn view(&self) -> iced::Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
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

impl TaskToIced for Task {
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

        let a_row = row(vec![])
            .spacing(10)
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(a_checkbox)
            .push(a_text_input)
            .push(delete_button);

        a_row.into()
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
                //self.data.tasks.remove(task_id);
                unreachable!();
            }
        }
    }
}
