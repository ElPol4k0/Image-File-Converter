use crate::app::message::Message;
use iced::widget::{column, container, pick_list, row, slider, text};
use iced::{Element, Length};

pub fn view(output_format: &str, quality: u8) -> Element<'_, Message> {
    container(
        column![
            text("Settings").size(20),
            row![
                text("Format:"),
                pick_list(
                    vec!["PNG", "JPEG", "WEBP", "GIF"],
                    Some(output_format),
                    |format| Message::FormatChanged(format.to_string())
                ),
            ]
            .spacing(10),
            row![
                text("Quality:"),
                slider(0..=100, quality, Message::QualityChanged),
                text(format!("{}%", quality)),
            ]
            .spacing(10),
        ]
        .spacing(15),
    )
    .width(Length::Fill)
    .padding(20)
    .into()
}
