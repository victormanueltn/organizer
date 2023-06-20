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
        let up_and_down = [
            iced::widget::Text::new("Up"),
            iced::widget::Text::new("Down"),
        ];

        let mut up_and_down = up_and_down
            .into_iter()
            .map(|text| {
                text.width(iced::Length::try_from(10).unwrap())
                    .height(6)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .size(10)
            })
            .collect::<Vec<iced::widget::Text>>();

        let up_button = {
            button(up_and_down.remove(0))
                .on_press(ListMessage::SwapWithPrevious)
                .padding(10)
        };

        let down_button = {
            button(up_and_down.remove(0))
                .on_press(ListMessage::SwapWithPrevious)
                .padding(10)
        };

        let up_and_down = column!(up_button, down_button);

        let mut a_column = column(vec![]);

        let messages = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| {
                (task.completed() && self.filters.complete)
                    || (task.visible_as_pending() && self.filters.todo)
            })
            .map(|(index, task)| {
                task.view()
                    .map(move |message| ListMessage::Task(index, message))
            })
            .collect::<Vec<_>>();

        for message in messages {
            a_column = a_column.push(message);
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
