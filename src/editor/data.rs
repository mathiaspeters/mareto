use iced::widget::text_editor;

use crate::{
    fs::{EntryType, FileSystemEntry},
    options::{DisplayType, Options, SortingOption},
};

#[derive(Debug, Default)]
pub struct EditorState {
    pub open_folder: Option<String>,
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
                    Some(
                        self.format_entry(
                            entry,
                            options
                                .display_type
                                .selected
                                .unwrap_or(DisplayType::RelativePath),
                        ),
                    )
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
        if options.min_depth.is_active && options.min_depth.limit.is_some_and(|l| entry.depth < l) {
            return false;
        }
        if options.max_depth.is_active && options.max_depth.limit.is_some_and(|l| entry.depth > l) {
            return false;
        }
        true
    }

    fn format_entry<'a>(
        &'_ self,
        entry: &'a FileSystemEntry,
        display_type: DisplayType,
    ) -> &'a str {
        match display_type {
            DisplayType::AbsolutePath => &entry.path,
            DisplayType::RelativePath => {
                let root = self
                    .open_folder
                    .as_ref()
                    .expect("Folder must be set for this function to be called");
                &entry
                    .path
                    .strip_prefix(root)
                    .expect("All available entries must start with the root")[1..]
            }
            DisplayType::JustName => entry
                .path
                .split('/')
                .last()
                .expect("Entries cannot be empty and must have at least one path separator"),
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
