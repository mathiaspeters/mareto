use iced::{widget::text_editor, Element, Length};

use crate::{mareto::Message, state::EditorState};

pub fn editor(state: &EditorState) -> Element<'_, Message> {
    text_editor(&state.contents)
        .on_action(Message::EditAction)
        .height(Length::Fill)
        .into()
}
