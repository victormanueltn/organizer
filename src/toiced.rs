use iced::{
    alignment,
    widget::{button, Button, Text},
    Element, Length,
};
pub(crate) trait ToIced {
    type Message;
    fn view(&self) -> Element<Self::Message>;
}

pub(crate) fn add_button<Message>(text: &str, message: Message) -> Button<Message> {
    let text = Text::new(text)
        .width(Length::Units(60))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20);
    button(text).on_press(message).padding(10)
}
