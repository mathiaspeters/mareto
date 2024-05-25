use iced::{
    widget::{button, row, text, text_input, Space},
    Element, Length,
};

use crate::mareto::Message;

pub fn find_and_replace(open_folder: &str) -> Element<'_, Message> {
    let s = String::new();
    row![
        text(open_folder),
        Space::with_width(Length::Fill),
        text_input("Find", &s).width(150).padding(8),
        button("Aa").padding(8), // Case sentivite
        button(".*").padding(8), // Use regex
        Space::with_width(8),
        text_input("Replace", &s).width(150).padding(8),
        button("a").padding(8), // Next occurence
        button("c").padding(8), // Previous occurence
        button("d").padding(8), // Replace current occurence
        button("r").padding(8), // Replace all
    ]
    .into()
}
