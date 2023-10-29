use crate::periodic_task;
use crate::toiced::ToIced;
use crate::{periodic_task::PeriodicTask, Organizer, ViewType};

#[derive(Debug, Clone)]
pub enum Message {
    SelectView(ViewType),
    Create,
    PeriodicTask(usize, periodic_task::Message),
}

pub(crate) trait PeriodicTasksManagementView {
    fn view_as_periodic_tasks_manager(&self) -> iced::Element<Message>;
    fn update_periodic_tasks_manager(&mut self, message: Message);
}

impl PeriodicTasksManagementView for Organizer {
    fn view_as_periodic_tasks_manager(&self) -> iced::Element<Message> {
        let view_pick_list =
            iced::widget::pick_list(&ViewType::ALL[..], self.view_type, Message::SelectView);

        let mut column = iced::widget::column(vec![]);
        column = column.push(view_pick_list);

        let periodic_tasks = self
            .data
            .periodic_tasks
            .iter()
            .enumerate()
            .map(move |(index, periodic_task)| {
                periodic_task
                    .view()
                    .map(move |message| Message::PeriodicTask(index, message))
            })
            .collect::<Vec<_>>();

        let create_text = iced::widget::Text::new("Add a new task")
            .width(iced::Length::try_from(120).unwrap())
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .size(20);
        let create_button = iced::widget::button(create_text)
            .on_press(Message::Create)
            .padding(10);

        for periodic_task in periodic_tasks {
            column = column.push(periodic_task);
        }

        column = column.push(create_button);

        column.align_items(iced::Alignment::Center).into()
    }

    fn update_periodic_tasks_manager(&mut self, message: Message) {
        match message {
            Message::SelectView(value) => self.view_type = Some(value),
            Message::Create => self
                .data
                .periodic_tasks
                .push(PeriodicTask::new("".to_string())),
            Message::PeriodicTask(index, message) => match message {
                periodic_task::Message::DeleteTask => _ = self.data.periodic_tasks.remove(index),
                _ => self.data.periodic_tasks[index].update(message),
            },
        }
    }
}
