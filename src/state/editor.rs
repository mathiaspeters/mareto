use iced::widget::text_editor;

use crate::{bit_set::BitSet, fs::FileSystemEntry};

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
        let visibility_vectors = filter_options.get_visibility_vectors();
        let mut is_visible = visibility_vectors[0].chunks.clone();
        for vector in visibility_vectors.iter().skip(1) {
            is_visible
                .iter_mut()
                .zip(vector.chunks.iter())
                .for_each(|(left, right)| *left &= right);
        }
        let is_visible = BitSet {
            chunks: is_visible,
            size: visibility_vectors[0].size,
        };
        let mut entries = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(i, entry)| {
                if is_visible.is_bit_set(i) {
                    let display_path = self.format_entry(
                        entry,
                        options
                            .display_type
                            .selected
                            .unwrap_or(DisplayType::RelativePath),
                    );
                    Some(display_path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        match options.sorting.selected {
            Some(SortingOption::SortAscending) => {
                entries.sort_unstable_by_key(|e| e.to_ascii_lowercase())
            }
            Some(SortingOption::SortDescending) => {
                entries.sort_unstable_by_key(|e| std::cmp::Reverse(e.to_ascii_lowercase()))
            }
            _ => {}
        }
        let content_size = entries.iter().map(|s| s.len()).sum();
        let mut content = String::with_capacity(content_size);
        entries.into_iter().for_each(|s| {
            if matches!(
                options
                    .display_type
                    .selected
                    .expect("There must be a display type"),
                DisplayType::AbsolutePath
            ) {
                content.push_str(
                    self.open_folder
                        .as_ref()
                        .expect("A folder must have been opened to get here"),
                );
                content.push('/');
            }
            content.push_str(s);
            content.push('\n');
        });
        self.contents = text_editor::Content::with_text(&content);
    }

    fn format_entry<'a>(
        &'_ self,
        entry: &'a FileSystemEntry,
        display_type: DisplayType,
    ) -> &'a str {
        match display_type {
            DisplayType::AbsolutePath | DisplayType::RelativePath => &entry.path,
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
