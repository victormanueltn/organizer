use std::collections::VecDeque;

use crate::fonts::icons::{downwards_arrow, upwards_arrow};
use crate::views::ListMessage;
use crate::{data::Data, toiced::ToIced};
use iced::{
    alignment,
    widget::{button, column, container, Column, Text},
    Element, Length,
};

impl ToIced for Data {
    type Message = ListMessage;
    fn view(&self) -> Element<Self::Message> {
        let create_swap_buttons = |index| {
            let up_and_down = [upwards_arrow(), downwards_arrow()];

            let mut up_and_down = up_and_down
                .into_iter()
                .map(|text| {
                    text.width(iced::Length::try_from(10).unwrap())
                        .height(6)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                        .size(10)
                })
                .collect::<VecDeque<iced::widget::Text>>();

            let up_button = {
                button(up_and_down.pop_front().unwrap())
                    .on_press(ListMessage::SwapWithPrevious(index))
                    .padding(10)
            };

            let down_button = {
                button(up_and_down.pop_front().unwrap())
                    .on_press(ListMessage::SwapWithNext(index))
                    .padding(10)
            };

            column!(up_button, down_button)
        };

        let mut a_column = column(vec![]);

        let messages = self
            .visible_tasks()
            .into_iter()
            .map(move |(index, task)| {
                task.view()
                    .map(move |message| ListMessage::Task(index, message))
            })
            .collect::<Vec<_>>();

        let tasks = messages
            .into_iter()
            .enumerate()
            .map(|(index, task)| iced::widget::row![create_swap_buttons(index), task])
            .collect::<VecDeque<_>>();

        for task in tasks {
            a_column = a_column.push(task);
        }

        a_column = add_task_button(a_column)
            .spacing(10)
            .align_items(iced::Alignment::Center);

        container(a_column).width(Length::Fill).center_x().into()
    }
}

fn add_task_button(a_column: Column<ListMessage>) -> Column<ListMessage> {
    let create_task_text = Text::new("Add a new task")
        .width(Length::try_from(120).unwrap())
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20);
    let edit_button = button(create_task_text)
        .on_press(ListMessage::AddTask)
        .padding(10);

    a_column
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .push(edit_button)
}
