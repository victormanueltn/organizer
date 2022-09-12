use iced::pure::Application;
use organizer::Organizer;

#[cfg(not(tarpaulin_include))]
fn main() -> iced::Result {
    Organizer::run(iced::Settings::default())
}
