use crate::ui::config::UiConfig;
use iced::daemon::Appearance;
use iced::window::{Position, Settings as WindowSettings};
use iced::{Color, Element, Font, Size, Task};

pub struct MyApp;

impl MyApp {
    pub fn new() -> (Self, Task<()>) {
        (Self, Task::none())
    }

    pub fn update(&mut self, _message: ()) -> Task<()> {
        Task::none()
    }

    pub fn view(&self) -> Element<()> {
        iced::widget::text("Hello World!").into()
    }

    // Window Settings
    pub fn window_settings() -> WindowSettings {
        let config = UiConfig::default();
        WindowSettings {
            size: Size::new(config.window_width, config.window_height),
            position: Position::Centered,
            ..Default::default()
        }
    }

    // Style Color
    pub fn style(_state: &Self, _theme: &iced::Theme) -> Appearance {
        let config = UiConfig::default();
        Appearance {
            background_color: config.background_color,
            text_color: Color::from_rgb(0.051, 0.067, 0.090),
        }
    }
}
