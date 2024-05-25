use iced::{
    widget::{button, column},
    Element, Length,
};

use crate::mareto::Message;

pub fn top_level_actions() -> Element<'static, Message> {
    let open_folder_button = top_level_button("Open folder", Message::OpenFolder);

    let apply_changes_button = top_level_button("Apply changes", Message::ApplyChanges);

    column![open_folder_button, apply_changes_button,]
        .spacing(12)
        .into()
}

fn top_level_button(label: &str, on_press: Message) -> Element<'_, Message> {
    button(label)
        .on_press(on_press)
        .width(Length::Fill)
        .padding(12)
        .into()
}
