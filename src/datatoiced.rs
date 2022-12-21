use crate::{
    data::{Data, Message},
    toiced::ToIced,
};
use iced::alignment;
use iced::{
    widget::{button, column, container, Column, Text},
    Element, Length,
};

impl ToIced for Data {
    type Message = Message;
    fn view(&self) -> Element<Self::Message> {
        let mut a_column = column(vec![]);

        for (index, task) in self.tasks.iter().enumerate() {
            a_column = a_column.push(
                task.view()
                    .map(move |message| Message::TaskMessage(index, message)),
            );
        }

        a_column = add_task_button(a_column)
            .spacing(10)
            .align_items(iced::Alignment::Start);

        container(a_column).width(Length::Fill).center_x().into()
    }
}

pub fn add_task_button(a_column: Column<Message>) -> Column<Message> {
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
