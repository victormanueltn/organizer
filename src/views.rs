use crate::toiced::ToIced;
use crate::{add_button, task, Organizer};
use crate::{Data, Text};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    List,
    Summary,
}

impl ViewType {
    pub const ALL: [ViewType; 2] = [ViewType::List, ViewType::Summary];
}

impl std::fmt::Display for ViewType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ViewType::List => "List",
                ViewType::Summary => "Summary",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ListViewMessage(ListMessage),
    SummaryViewMessage(SummaryMessage),
}

#[derive(Debug, Clone)]
pub enum ListMessage {
    AddTask,
    Task(usize, task::Message),
    Load,
    UpdateSaveFileName(String),
    Save,
    ToggleActiveFilter(bool),
    ToggleCompleteFilter(bool),
    SelectView(ViewType),
}

#[derive(Debug, Clone)]
pub enum SummaryMessage {
    SelectView(ViewType),
    UpdateInitialDay(String),
    UpdateInitialMonth(String),
    UpdateInitialYear(String),
    UpdateFinalDay(String),
    UpdateFinalMonth(String),
    UpdateFinalYear(String),
}

pub(crate) trait ListView {
    fn view_as_list(&self) -> iced::Element<ListMessage>;
    fn update_list_view(&mut self, message: ListMessage);
}

impl ListView for Organizer {
    fn view_as_list(&self) -> iced::Element<ListMessage> {
        let button_todo_tasks = iced::widget::Checkbox::new(
            "Todo",
            self.data.filters.todo,
            ListMessage::ToggleActiveFilter,
        );
        let button_complete_tasks = iced::widget::Checkbox::new(
            "Complete",
            self.data.filters.complete,
            ListMessage::ToggleCompleteFilter,
        );

        let a_row = iced::widget::row![button_todo_tasks, button_complete_tasks].spacing(40);

        let data_view = self.data.view();
        let mut a_column =
            iced::widget::column(vec![a_row.into()]).align_items(iced::Alignment::Center);
        if let Some(ref error_text) = self.error_text {
            a_column = a_column
                .push(Text::new(error_text).style(iced::Color::from_rgb(1., 0., 0.)))
                .align_items(iced::Alignment::Center);
        }

        let file_name_input = iced::widget::text_input(
            "Name of the task list",
            &self.file_name,
            ListMessage::UpdateSaveFileName,
        )
        .padding(10);
        let load_button = add_button("Save task list", ListMessage::Save);
        let save_button = add_button("Load task list", ListMessage::Load);
        let a_row = iced::widget::row!(file_name_input, save_button, load_button)
            .spacing(10)
            .padding(10);

        let view_pick_list =
            iced::widget::pick_list(&ViewType::ALL[..], self.view_type, ListMessage::SelectView);

        a_column
            .push(a_row)
            .push(data_view)
            .push(view_pick_list)
            .spacing(10)
            .into()
    }

    fn update_list_view(&mut self, message: ListMessage) {
        match message {
            ListMessage::AddTask => self.add_task(),
            ListMessage::Task(task_id, task_message) => {
                if task_id > self.data.tasks.len() {
                    panic!("Tried to update inexisting task.")
                };
                self.process_task_message(task_id, task_message)
            }
            ListMessage::UpdateSaveFileName(file_name) => {
                self.file_name = file_name;
            }
            ListMessage::Load => {
                let loaded_data = Data::load(&self.file_name);
                match loaded_data {
                    Ok(loaded_data) => self.data = loaded_data,
                    Err(error) => {
                        self.error_text =
                            Some(format!("{0:?} problem: {1:?}", error.kind, error.message))
                    }
                }
            }
            ListMessage::Save => {
                let save_result = self.data.save(&self.file_name);
                if let Err(error) = save_result {
                    self.error_text =
                        Some(format!("{0:?} problem: {1:?}", error.kind, error.message));
                }
            }
            ListMessage::ToggleActiveFilter(value) => {
                self.data.filters.todo = value;
            }
            ListMessage::ToggleCompleteFilter(value) => {
                self.data.filters.complete = value;
            }
            ListMessage::SelectView(value) => self.view_type = Some(value),
        }
    }
}
