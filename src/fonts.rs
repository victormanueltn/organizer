pub mod icons {

    pub(crate) fn upwards_arrow() -> iced::widget::Text<'static> {
        icon('\u{E800}')
    }

    pub(crate) fn downwards_arrow() -> iced::widget::Text<'static> {
        icon('\u{E801}')
    }

    fn icon(unicode: char) -> iced::widget::Text<'static> {
        iced::widget::text(unicode.to_string())
            .font(ARROWS)
            .width(20)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
    }

    const ARROWS: iced::Font = iced::Font::External {
        // Created in fontello.com. See it's contents in fontdrop.info
        name: "Arrows",
        bytes: include_bytes!("../fonts/arrows.ttf"),
    };
}
