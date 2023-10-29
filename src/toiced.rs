use iced::{
    alignment,
    widget::{button, Button, Text},
    Element, Length,
};
pub(crate) trait ToIced {
    type Message;
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
}

pub(crate) fn add_button<Message>(text: &str, message: Message) -> Button<Message> {
    let text = Text::new(text)
        .width(Length::try_from(60).unwrap())
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20);
    button(text).on_press(message).padding(10)
}
