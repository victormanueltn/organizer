use crate::task;

#[derive(Debug, Clone)]
pub enum Message {
    ListViewMessage(ListMessage),
    //ViewMessage(summary::Message),
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
}
