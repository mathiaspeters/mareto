use iced::widget::{button, column, row, text, text_editor, text_input, Space};
use iced::{Element, Length};

use crate::mareto::Message;

use super::EditorState;

pub fn editor_view(state: &EditorState) -> Element<'_, Message> {
    let folder = state.open_folder.as_deref().unwrap_or("No folder open");
    let s = String::new();
    column![
        text_editor(&state.contents)
            .on_action(Message::EditAction)
            .height(Length::Fill),
        row![
            text(folder),
            Space::with_width(Length::Fill),
            text_input("Find", &s).width(150).padding(8),
            button(".*").padding(8), // Use regex
            Space::with_width(8),
            text_input("Replace", &s).width(150).padding(8),
            button("a").padding(8), // Next occurence
            button("c").padding(8), // Previous occurence
            button("d").padding(8), // Replace current occurence
            button("r").padding(8), // Replace all
        ]
    ]
    .spacing(12)
    .into()
}
