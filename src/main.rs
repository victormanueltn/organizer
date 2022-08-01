use iced::pure::Application;
use organizer::Organizer;

fn main() -> iced::Result {
    Organizer::run(iced::Settings::default())
}
