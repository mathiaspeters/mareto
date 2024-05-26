use iced::widget::text_editor;

use crate::fs::{EntryType, FileSystemEntry};

use super::{DisplayType, FilterOptions, Options, SortingOption};

#[derive(Debug, Default)]
pub struct EditorState {
    pub open_folder: Option<String>,
    pub entries: Vec<FileSystemEntry>,

    pub contents: text_editor::Content,
    pub find_and_replace: FindAndReplace,
}

impl EditorState {
    pub fn show_filtered_entries(&mut self, options: &Options, filter_options: &FilterOptions) {
        let re = match &filter_options.filter_input.state.regex {
            Some(Ok(re)) => Some(re),
            Some(Err((re, _))) => re.as_ref(),
            _ => None,
        };
        let mut content = self
            .entries
            .iter()
            .filter_map(|entry| {
                if Self::entry_is_visible(entry, filter_options) {
                    let display_path = self.format_entry(
                        entry,
                        options
                            .display_type
                            .selected
                            .unwrap_or(DisplayType::RelativePath),
                    );
                    match &re {
                        Some(re) if re.is_match(&display_path) => Some(display_path),
                        None => {
                            if filter_options.filter_input.state.case_sensitive {
                                if display_path.contains(&filter_options.filter_input.state.input) {
                                    Some(display_path)
                                } else {
                                    None
                                }
                            } else if display_path
                                .to_lowercase()
                                .contains(&filter_options.filter_input.state.input.to_lowercase())
                            {
                                Some(display_path)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        match options.sorting.selected {
            Some(SortingOption::SortAscending) => content
                .sort_unstable_by(|e1, e2| e1.to_ascii_lowercase().cmp(&e2.to_ascii_lowercase())),
            Some(SortingOption::SortDescending) => content
                .sort_unstable_by(|e1, e2| e2.to_ascii_lowercase().cmp(&e1.to_ascii_lowercase())),
            _ => {}
        }
        let content = content.join("\n");
        self.contents = text_editor::Content::with_text(&content);
    }

    fn entry_is_visible(entry: &FileSystemEntry, filter_options: &FilterOptions) -> bool {
        if (matches!(entry.entry_type, EntryType::File) && !filter_options.show_files.state)
            || (matches!(entry.entry_type, EntryType::Folder) && !filter_options.show_folders.state)
        {
            return false;
        }
        if filter_options.min_depth.state.is_active
            && filter_options
                .min_depth
                .state
                .limit
                .is_some_and(|l| entry.depth < l)
        {
            return false;
        }
        if filter_options.max_depth.state.is_active
            && filter_options
                .max_depth
                .state
                .limit
                .is_some_and(|l| entry.depth > l)
        {
            return false;
        }
        true
    }

    fn format_entry<'a>(&'_ self, entry: &'a FileSystemEntry, display_type: DisplayType) -> String {
        match display_type {
            DisplayType::AbsolutePath => format!(
                "{}{}",
                self.open_folder
                    .as_ref()
                    .expect("Folder must be set for this function to be called"),
                &entry.path
            ),
            DisplayType::RelativePath => entry.path.to_owned(),
            DisplayType::JustName => entry
                .path
                .split('/')
                .last()
                .expect("Entries cannot be empty and must have at least one path separator")
                .to_owned(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FindAndReplace {
    pub find: String,
    pub replace: String,
    pub current_occurence: usize,
    pub occurences: Vec<FoundOccurence>,
}

#[derive(Debug, Clone)]
pub struct FoundOccurence {
    pub line: usize,
    pub start: usize,
    pub len: usize,
}
