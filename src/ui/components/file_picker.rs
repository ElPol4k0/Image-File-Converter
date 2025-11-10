use crate::app::message::Message;
use crate::ui::styles; // ← Hinzufügen!
use iced::widget::{button, row, text};
use iced::{Element, Length};
use std::path::PathBuf;

pub fn view(selected: &Option<PathBuf>) -> Element<'static, Message> {
    let file_text = match selected {
        Some(path) => path.to_string_lossy().to_string(),
        None => "No file selected".to_string(),
    };

    row![
        text(file_text).width(Length::Fill),
        button("Browse...")
            .on_press(Message::SelectFilePressed)
            .style(styles::button::custom_primary)
    ]
    .spacing(10)
    .padding(10)
    .into()
}
