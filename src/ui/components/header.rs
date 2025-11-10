use crate::app::message::Message;
use iced::color;
use iced::widget::{column, container, text};
use iced::Color;
use iced::{Alignment, Element, Length};

pub fn view() -> Element<'static, Message> {
    container(
        column![
            text("Image Converter").size(32).color(color!(0xbe29ec)),
            text("Convert your images easily")
                .size(14)
                .color(Color::WHITE)
        ]
        .spacing(5)
        .align_x(Alignment::Center)
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(20)
    .into()
}
