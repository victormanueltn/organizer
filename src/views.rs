pub(crate) mod list_view;
pub(crate) mod periodic_tasks_management;
pub(crate) mod summary_view;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    List,
    Summary,
    PeriodicTasksManagement,
}

impl ViewType {
    pub const ALL: [ViewType; 3] = [
        ViewType::List,
        ViewType::Summary,
        ViewType::PeriodicTasksManagement,
    ];
}

impl std::fmt::Display for ViewType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ViewType::List => "List",
                ViewType::Summary => "Summary",
                ViewType::PeriodicTasksManagement => "Periodic Tasks Management",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    List(list_view::Message),
    Summary(summary_view::Message),
    PeriodicTasksManagement(periodic_tasks_management::Message),
}
