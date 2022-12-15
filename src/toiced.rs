use iced::Element;
pub(crate) trait ToIced {
    type Message;
    fn view(&self) -> Element<Self::Message>;
}
