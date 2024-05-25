use iced::{
    executor,
    widget::{column, container, row, rule::Rule, text, text_editor},
    Application, Command, Element, Theme,
};

use crate::{fs::get_entries_for_path, state::EditorState, ui};
use crate::{
    fs::FileSystemEntry,
    state::{DisplayType, Options, SortingOption},
};

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IOError(std::io::ErrorKind),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.kind())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    // Top-level actions
    OpenFolder,
    FolderSelected(Result<(String, Vec<FileSystemEntry>), Error>),
    ApplyChanges,
    //ApplyOutcome(Result<(), Error>),

    // Options updates
    FilterUpdated(String),
    FilterCaseSensitivityToggled,
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

    fn theme(&self) -> Theme {
        self.options
            .theme
            .selected
            .clone()
            .unwrap_or(Theme::default())
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            // Top-level actions
            Message::OpenFolder => Command::perform(pick_folder(), Message::FolderSelected),
            Message::FolderSelected(Ok((path, entries))) => {
                self.editor_state.open_folder = Some(path);
                self.editor_state.entries = entries;
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::FolderSelected(_) => Command::none(),
            Message::ApplyChanges => Command::none(),
            //Message::ApplyOutcome(_) => Command::none(),

            // Options updates
            Message::FilterUpdated(filter) => {
                self.options.filter_input.input = filter;
                self.options.filter_input.update_regex();
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::FilterRegexToggled => {
                self.options.filter_input.use_regex = !self.options.filter_input.use_regex;
                self.options.filter_input.update_regex();
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::FilterCaseSensitivityToggled => {
                self.options.filter_input.case_sensitive =
                    !self.options.filter_input.case_sensitive;
                self.options.filter_input.update_regex();
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::MinDepthToggled(is_active) => {
                self.options.min_depth.is_active = is_active;
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::MinDepthLimitChanged(mut limit) => {
                limit.retain(|c| c.is_numeric());
                self.options.min_depth.limit = if limit.is_empty() {
                    None
                } else {
                    Some(limit.parse().expect("Only numbers should still be there"))
                };
                match (self.options.min_depth.limit, self.options.max_depth.limit) {
                    (Some(min_limit), Some(max_limit)) if max_limit < min_limit => {
                        self.options.max_depth.limit = Some(min_limit);
                    }
                    _ => {}
                }
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::MaxDepthToggled(is_active) => {
                self.options.max_depth.is_active = is_active;
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::MaxDepthLimitChanged(mut limit) => {
                limit.retain(|c| c.is_numeric());
                self.options.max_depth.limit = if limit.is_empty() {
                    None
                } else {
                    Some(limit.parse().expect("Only numbers should still be there"))
                };
                match (self.options.min_depth.limit, self.options.max_depth.limit) {
                    (Some(min_limit), Some(max_limit)) if min_limit > max_limit => {
                        self.options.min_depth.limit = Some(max_limit);
                    }
                    _ => {}
                }
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::ShowFilesToggled(is_active) => {
                self.options.show_files = is_active;
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::ShowFoldersToggled(is_active) => {
                self.options.show_folders = is_active;
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::SortOrderSelected(order) => {
                self.options.sorting.selected = Some(order);
                self.editor_state.show_filtered_entries(&self.options);
                Command::none()
            }
            Message::DisplayTypeSelected(display_type) => {
                self.options.display_type.selected = Some(display_type);
                self.editor_state.show_filtered_entries(&self.options);
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
            Message::EditAction(action) => {
                self.editor_state.contents.perform(action);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let left_pane = column![
            ui::top_level_actions(),
            Rule::horizontal(1),
            text("Options"),
            ui::options(&self.options),
        ]
        .width(400)
        .spacing(12);

        let right_pane = column![
            ui::editor(&self.editor_state),
            ui::find_and_replace(
                self.editor_state
                    .open_folder
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("")
            ),
        ]
        .spacing(12);

        container(row![left_pane, right_pane].spacing(12))
            .padding(12)
            .into()
    }
}

async fn pick_folder() -> Result<(String, Vec<FileSystemEntry>), Error> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("Choose a folder...")
        .pick_folder()
        .await
        .and_then(|fh| fh.path().to_str().map(|s| s.to_owned()))
        .ok_or(Error::DialogClosed)?;
    let entries = get_entries_for_path(&path)?;

    Ok((path, entries))
}
