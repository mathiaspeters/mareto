mod bit_set;
mod fs;
mod mareto;
mod state;
mod ui;

use iced::{Application, Settings, Size};

use crate::mareto::Mareto;

fn main() -> iced::Result {
    Mareto::run(Settings {
        window: iced::window::Settings {
            min_size: Some(Size::new(1280.0, 720.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}
