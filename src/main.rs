use iced::Sandbox;

mod task;
use task::Task;

fn main() -> iced::Result {
    Task::run(iced::Settings::default())
}
