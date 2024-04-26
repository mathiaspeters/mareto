use std::path::PathBuf;

use iced::widget::text_editor;

use crate::{
    fs::{EntryType, FileSystemEntry},
    options::{Options, SortingOption},
};

#[derive(Debug, Default)]
pub struct EditorState {
    pub open_folder: Option<PathBuf>,
    pub entries: Vec<FileSystemEntry>,

    pub contents: text_editor::Content,
    pub find_and_replace: FindAndReplace,
}

impl EditorState {
    pub fn show_filtered_entries(&mut self, options: &Options) {
        let mut content = self
            .entries
            .iter()
            .filter_map(|entry| {
                if Self::entry_is_visible(entry, options) {
                    Some(entry.path.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        content.sort_unstable_by(|e1, e2| match options.sorting.selected {
            Some(SortingOption::Descending) => {
                e2.to_ascii_lowercase().cmp(&e1.to_ascii_lowercase())
            }
            _ => e1.to_ascii_lowercase().cmp(&e2.to_ascii_lowercase()),
        });
        let content = content.join("\n");
        self.contents = text_editor::Content::with_text(&content);
    }

    fn entry_is_visible(entry: &FileSystemEntry, options: &Options) -> bool {
        if (matches!(entry.entry_type, EntryType::File) && !options.show_files)
            || (matches!(entry.entry_type, EntryType::Folder) && !options.show_folders)
        {
            return false;
        }
        true
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
