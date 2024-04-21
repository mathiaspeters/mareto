//use std::path::PathBuf;

use iced::{
    executor,
    widget::{button, column, container, row, rule::Rule, text, text_editor},
    Application, Command, Element, Length, Theme,
};

use crate::options::{options_view, DisplayType, Options};
use crate::{
    editor::{editor_view, EditorState},
    options::SortingOption,
};

//#[derive(Debug, Clone)]
//enum Error {
//    IOError(std::io::ErrorKind),
//}

#[derive(Debug, Clone)]
pub enum Message {
    // Top-level actions
    OpenFolder,
    //FolderSelected(Result<PathBuf, Error>),
    ApplyChanges,
    //ApplyOutcome(Result<(), Error>),

    // Options updates
    FilterUpdated(String),
    FilterRegexToggled,
    MinDepthToggled(bool),
    MinDepthLimitChanged(String),
    MaxDepthToggled(bool),
    MaxDepthLimitChanged(String),
    ShowFilesToggled(bool),
    ShowFoldersToggled(bool),
    SortOrderSelected(SortingOption),
    DisplayTypeSelected(DisplayType),
    RemoveFoldersToggled(bool),
    PreviewChangesToggled(bool),
    ThemeSelected(Theme),

    // Editor actions
    EditAction(text_editor::Action),
}

#[derive(Debug, Default)]
pub struct Mareto {
    options: Options,
    editor_state: EditorState,
}

impl Application for Mareto {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mareto = Self::default();
        (mareto, Command::none())
    }

    fn title(&self) -> String {
        "Mareto".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            // Top-level actions
            Message::OpenFolder => Command::none(),
            //Message::FolderSelected(_) => Command::none(),
            Message::ApplyChanges => Command::none(),
            //Message::ApplyOutcome(_) => Command::none(),

            // Options updates
            Message::FilterUpdated(filter) => {
                self.options.filter_input.input = filter;
                Command::none()
            }
            Message::FilterRegexToggled => {
                self.options.filter_input.use_regex = !self.options.filter_input.use_regex;
                Command::none()
            }
            Message::MinDepthToggled(is_active) => {
                self.options.min_depth.is_active = is_active;
                Command::none()
            }
            Message::MinDepthLimitChanged(mut limit) => {
                limit.retain(|c| c.is_numeric());
                self.options.min_depth.limit =
                    limit.parse().expect("Only numbers should still be there");
                Command::none()
            }
            Message::MaxDepthToggled(is_active) => {
                self.options.max_depth.is_active = is_active;
                Command::none()
            }
            Message::MaxDepthLimitChanged(mut limit) => {
                limit.retain(|c| c.is_numeric());
                self.options.max_depth.limit =
                    limit.parse().expect("Only numbers should still be there");
                Command::none()
            }
            Message::ShowFilesToggled(is_active) => {
                self.options.show_files = is_active;
                Command::none()
            }
            Message::ShowFoldersToggled(is_active) => {
                self.options.show_folders = is_active;
                Command::none()
            }
            Message::SortOrderSelected(order) => {
                self.options.sorting.selected = Some(order);
                Command::none()
            }
            Message::DisplayTypeSelected(display_type) => {
                self.options.display_type.selected = Some(display_type);
                Command::none()
            }
            Message::RemoveFoldersToggled(is_active) => {
                self.options.remove_empty = is_active;
                Command::none()
            }
            Message::PreviewChangesToggled(is_active) => {
                self.options.preview_changes = is_active;
                Command::none()
            }
            Message::ThemeSelected(theme) => {
                self.options.theme.selected = Some(theme);
                Command::none()
            }

            // Editor actions
            Message::EditAction(_) => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let left_pane = {
            let open_folder_button = top_level_button("Open folder", Message::OpenFolder);
            let apply_changes_button = top_level_button("Apply changes", Message::ApplyChanges);

            let options = options_view(&self.options);

            column![
                open_folder_button,
                apply_changes_button,
                Rule::horizontal(1),
                text("Options"),
                options
            ]
            .width(400)
            .spacing(12)
        };

        let right_pane = editor_view(&self.editor_state);

        container(row![left_pane, right_pane].spacing(12))
            .padding(12)
            .into()
    }
}

fn top_level_button(label: &str, on_press: Message) -> Element<'_, Message> {
    button(label)
        .on_press(on_press)
        .width(Length::Fill)
        .padding(12)
        .into()
}