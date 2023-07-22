use crate::{Organizer, ViewType};

#[derive(Debug, Clone)]
pub enum Message {
    SelectView(ViewType),
}

pub(crate) trait PeriodicTasksManagementView {
    fn view_as_periodic_tasks_manager(&self) -> iced::Element<Message>;
    fn update_periodic_tasks_manager(&mut self, message: Message);
}

impl PeriodicTasksManagementView for Organizer {
    fn view_as_periodic_tasks_manager(&self) -> iced::Element<Message> {
        let view_pick_list =
            iced::widget::pick_list(&ViewType::ALL[..], self.view_type, Message::SelectView);

        let mut a_column = iced::widget::column(vec![]);

        a_column = a_column.push(view_pick_list);

        a_column.into()
    }

    fn update_periodic_tasks_manager(&mut self, message: Message) {
        match message {
            Message::SelectView(value) => self.view_type = Some(value),
        }
    }


    // - Initial date (and time?)
    // - Frequency (x times per day/week/month/year)
}
