use iced::{
    executor,
    widget::{column, container, row, rule::Rule, text, text_editor},
    Application, Command, Element, Theme,
};

use crate::{
    fs::get_entries_for_path,
    state::{EditorState, FilterOptions},
    ui,
};
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
    filters: FilterOptions,
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
        self.options.theme.selected.clone().unwrap_or_default()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            // Top-level actions
            Message::OpenFolder => Command::perform(pick_folder(), Message::FolderSelected),
            Message::FolderSelected(Ok((path, entries))) => {
                self.editor_state.open_folder = Some(path);
                self.editor_state.entries = entries;
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::FolderSelected(_) => Command::none(),
            Message::ApplyChanges => Command::none(),
            //Message::ApplyOutcome(_) => Command::none(),

            // Options updates
            Message::FilterUpdated(filter) => {
                self.filters.filter_input.state.input = filter;
                self.filters.filter_input.state.update_regex();
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::FilterRegexToggled => {
                self.filters.filter_input.state.use_regex =
                    !self.filters.filter_input.state.use_regex;
                self.filters.filter_input.state.update_regex();
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::FilterCaseSensitivityToggled => {
                self.filters.filter_input.state.case_sensitive =
                    !self.filters.filter_input.state.case_sensitive;
                self.filters.filter_input.state.update_regex();
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::MinDepthToggled(is_active) => {
                self.filters.min_depth.state.is_active = is_active;
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::MinDepthLimitChanged(mut limit) => {
                limit.retain(|c| c.is_numeric());
                self.filters.min_depth.state.limit = if limit.is_empty() {
                    None
                } else {
                    Some(limit.parse().expect("Only numbers should still be there"))
                };
                match (
                    self.filters.min_depth.state.limit,
                    self.filters.max_depth.state.limit,
                ) {
                    (Some(min_limit), Some(max_limit)) if max_limit < min_limit => {
                        self.filters.max_depth.state.limit = Some(min_limit);
                    }
                    _ => {}
                }
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::MaxDepthToggled(is_active) => {
                self.filters.max_depth.state.is_active = is_active;
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::MaxDepthLimitChanged(mut limit) => {
                limit.retain(|c| c.is_numeric());
                self.filters.max_depth.state.limit = if limit.is_empty() {
                    None
                } else {
                    Some(limit.parse().expect("Only numbers should still be there"))
                };
                match (
                    self.filters.min_depth.state.limit,
                    self.filters.max_depth.state.limit,
                ) {
                    (Some(min_limit), Some(max_limit)) if min_limit > max_limit => {
                        self.filters.min_depth.state.limit = Some(max_limit);
                    }
                    _ => {}
                }
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::ShowFilesToggled(is_active) => {
                self.filters.show_files.state = is_active;
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::ShowFoldersToggled(is_active) => {
                self.filters.show_folders.state = is_active;
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::SortOrderSelected(order) => {
                self.options.sorting.selected = Some(order);
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
                Command::none()
            }
            Message::DisplayTypeSelected(display_type) => {
                self.options.display_type.selected = Some(display_type);
                self.editor_state
                    .show_filtered_entries(&self.options, &self.filters);
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
            ui::options(&self.options, &self.filters),
        ]
        .width(400)
        .spacing(12);

        let right_pane = column![
            ui::editor(&self.editor_state),
            ui::find_and_replace(self.editor_state.open_folder.as_deref().unwrap_or("")),
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
