mod app;
mod core;
mod types;
mod ui;

use app::state::MyApp;
use iced::{application, Result};

fn main() -> Result {
    application("Image Converter", MyApp::update, MyApp::view)
        .window(MyApp::window_settings())
        .style(MyApp::style)
        .run_with(MyApp::new)
}
