use crate::{
    data::{Data, Message},
    toiced::ToIced,
};
use iced::{
    alignment,
    widget::{button, column, container, Column, Text},
    Element, Length,
};

impl ToIced for Data {
    type Message = Message;
    fn view(&self) -> Element<Self::Message> {
        let mut a_column = column(vec![]);

        let messages: Vec<_> = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| {
                (task.completed() && self.filters.complete)
                    || (!task.completed() && self.filters.todo)
            })
            .map(|(index, task)| {
                task.view()
                    .map(move |message| Message::Task(index, message))
            })
            .collect();

        for message in messages {
            a_column = a_column.push(message);
        }

        a_column = add_task_button(a_column)
            .spacing(10)
            .align_items(iced::Alignment::Center);

        container(a_column).width(Length::Fill).center_x().into()
    }
}

fn add_task_button(a_column: Column<Message>) -> Column<Message> {
    let create_task_text = Text::new("Add a new task")
        .width(Length::Units(120))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20);
    let edit_button = button(create_task_text)
        .on_press(Message::AddTask)
        .padding(10);

    a_column
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .push(edit_button)
}
