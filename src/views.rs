use crate::task;

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
}
