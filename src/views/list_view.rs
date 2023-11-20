use crate::datatoiced::DataToIced;
use crate::toiced::ToIced;
use crate::ViewType;
use crate::{add_button, task, Organizer};
use crate::{Data, Text};

#[derive(Debug, Clone)]
pub enum Message {
    AddTask,
    Task(usize, task::Message),
    Load,
    UpdateSaveFileName(String),
    Save,
    ToggleActiveFilter(bool),
    ToggleCompleteFilter(bool),
    SwapWithPrevious(usize),
    SwapWithNext(usize),
    SelectView(ViewType),
}

pub(crate) trait ListView {
    fn view_as_list(&self) -> iced::Element<Message>;
    fn update_list_view(&mut self, message: Message);
}

impl ListView for Organizer {
    fn view_as_list(&self) -> iced::Element<Message> {
        let button_todo_tasks = iced::widget::Checkbox::new(
            "Todo",
            self.data.filters.todo,
            Message::ToggleActiveFilter,
        );
        let button_complete_tasks = iced::widget::Checkbox::new(
            "Complete",
            self.data.filters.complete,
            Message::ToggleCompleteFilter,
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

        let file_name = &self.file_name.clone().unwrap_or(String::new());
        let file_name_input = iced::widget::text_input(
            "Name of the task list",
            file_name,
            Message::UpdateSaveFileName,
        )
        .padding(10);
        let load_button = add_button("Save task list", Message::Save);
        let save_button = add_button("Load task list", Message::Load);
        let a_row = iced::widget::row!(file_name_input, save_button, load_button)
            .spacing(10)
            .padding(10);

        let view_pick_list =
            iced::widget::pick_list(&ViewType::ALL[..], self.view_type, Message::SelectView);

        a_column
            .push(a_row)
            .push(data_view)
            .push(view_pick_list)
            .spacing(10)
            .into()
    }

    fn update_list_view(&mut self, message: Message) {
        self.data
            .periodic_tasks
            .iter_mut()
            .map(|periodic_task| periodic_task.create_tasks())
            .enumerate()
            .for_each(|(index, tasks)| {
                let size = self.data.tasks.len();
                tasks.into_iter().for_each(|mut task| {
                    task.id = size + index + 1usize;
                    self.data.tasks.push(task);
                })
            });

        match message {
            Message::AddTask => self.add_task(),
            Message::Task(task_id, task_message) => {
                if task_id > self.data.tasks.len() {
                    panic!("Tried to update inexisting task.")
                };
                if let task::Message::DeleteTask = task_message {
                    self.data.tasks.remove(task_id);
                } else {
                self.data.tasks[task_id].update(task_message);
                }
            }
            Message::UpdateSaveFileName(file_name) => {
                self.file_name = Some(file_name);
            }
            Message::Load => {
                let loaded_data = Data::load(&self.file_name.clone().unwrap_or(String::new()));
                match loaded_data {
                    Ok(loaded_data) => self.data = loaded_data,
                    Err(error) => {
                        self.error_text =
                            Some(format!("{0:?} problem: {1:?}", error.kind, error.message))
                    }
                }
            }
            Message::Save => {
                let save_result = self.data.save(self.file_name.as_ref().unwrap());
                if let Err(error) = save_result {
                    self.error_text =
                        Some(format!("{0:?} problem: {1:?}", error.kind, error.message));
                }
            }
            Message::ToggleActiveFilter(value) => {
                self.data.filters.todo = value;
            }
            Message::ToggleCompleteFilter(value) => {
                self.data.filters.complete = value;
            }
            Message::SwapWithPrevious(index) => {
                let first_visible = index == 0;
                if !first_visible {
                    let visible_tasks = self.data.visible_tasks();
                    let (current_index, _) = visible_tasks[index];
                    let (previous_index, _) = visible_tasks[index - 1];
                    self.data.tasks.swap(current_index, previous_index);
                }
            }
            Message::SwapWithNext(index) => {
                let visible_tasks = self.data.visible_tasks();
                let last_visible = index + 1 == visible_tasks.len();
                if !last_visible {
                    let (current_index, _) = visible_tasks[index];
                    let (next_index, _) = visible_tasks[index + 1];
                    self.data.tasks.swap(current_index, next_index);
                }
            }

            Message::SelectView(value) => self.view_type = Some(value),
        }
    }
}
