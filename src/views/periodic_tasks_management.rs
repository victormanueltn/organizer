use crate::{Organizer, ViewType};

#[derive(Debug, Clone)]
pub enum PeriodicTasksManagementMessage {
    SelectView(ViewType),
}

pub(crate) trait PeriodicTasksManagementView {
    fn view_as_periodic_tasks_manager(&self) -> iced::Element<PeriodicTasksManagementMessage>;
    fn update_periodic_tasks_manager(&mut self, message: PeriodicTasksManagementMessage);
}

impl PeriodicTasksManagementView for Organizer {
    fn view_as_periodic_tasks_manager(&self) -> iced::Element<PeriodicTasksManagementMessage> {
        let view_pick_list = iced::widget::pick_list(
            &ViewType::ALL[..],
            self.view_type,
            PeriodicTasksManagementMessage::SelectView,
        );

        let mut a_column = iced::widget::column(vec![]);

        a_column = a_column.push(view_pick_list);

        a_column.into()
    }

    fn update_periodic_tasks_manager(&mut self, message: PeriodicTasksManagementMessage) {
        match message {
            PeriodicTasksManagementMessage::SelectView(value) => self.view_type = Some(value),
        }
    }
}
