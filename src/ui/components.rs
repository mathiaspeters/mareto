use iced::widget::button;

use crate::mareto::Message;

use super::themes::InactiveToggleButton;

pub fn toggle_button(
    label: &str,
    active: bool,
    on_press: Message,
) -> iced::widget::Button<'_, Message> {
    let mut button = button(label).padding(12).on_press(on_press);
    if !active {
        button = button.style(InactiveToggleButton);
    }
    button
}
