use crate::app::message::Message;
use iced::widget::{button, column, container, text};
use iced::{Element, Length};

pub fn view(status: &str) -> Element<'_, Message> {
    container(
        column![
            button("Convert Image")
                .on_press(Message::ConvertPressed)
                .style(button::success)
                .padding(15)
                .width(Length::Fill),
            text(status).size(14),
        ]
        .spacing(10),
    )
    .width(Length::Fill)
    .padding(20)
    .into()
}
