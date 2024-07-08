use iced::widget::text_editor;

use crate::{bit_set::BitSet, fs::FileSystemEntry};

use super::{DisplayType, FilterOptions, Options, SortingOption};

#[derive(Debug, Default)]
pub struct EditorState {
    pub open_folder: Option<String>,
    pub entries: Vec<FileSystemEntry>,
    pub filtered_indices: Vec<usize>,

    pub contents: text_editor::Content,
    pub find_and_replace: FindAndReplace,
}

impl EditorState {
    pub fn handle_action(&mut self, action: text_editor::Action) {
        match action {
            text_editor::Action::Edit(edit) if !self.handle_edit(&edit) => {}
            _ => self.contents.perform(action),
        }
    }

    fn handle_edit(&mut self, edit: &text_editor::Edit) -> bool {
        let (row, col) = self.contents.cursor_position();
        match edit {
            text_editor::Edit::Insert(_) => true,
            text_editor::Edit::Paste(_) => true,
            text_editor::Edit::Backspace => col > 0,
            text_editor::Edit::Delete => {
                col < self
                    .contents
                    .line(row)
                    .expect(
                        "The contents cursor position cannot point to a row that does not exist",
                    )
                    .len()
            }
            text_editor::Edit::Enter => false,
        }
    }

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
        let mut content_size = 0;
        let (filtered, entries): (Vec<usize>, Vec<&str>) = self
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
                    content_size += display_path.len();
                    Some((i, display_path))
                } else {
                    None
                }
            })
            .unzip();
        let mut filtered = filtered.into_iter().enumerate().collect::<Vec<_>>();
        match options.sorting.selected {
            Some(SortingOption::SortAscendingCaseInsensitive) => {
                filtered.sort_unstable_by_key(|(_, i)| entries[*i].to_ascii_lowercase())
            }
            Some(SortingOption::SortAscendingCaseSensitive) => {
                filtered.sort_unstable_by_key(|(_, i)| entries[*i])
            }
            Some(SortingOption::SortDescendingCaseInsensitive) => filtered
                .sort_unstable_by_key(|(_, i)| std::cmp::Reverse(entries[*i].to_ascii_lowercase())),
            Some(SortingOption::SortDescendingCaseSensitive) => {
                filtered.sort_unstable_by_key(|(_, i)| std::cmp::Reverse(entries[*i]))
            }
            _ => {}
        }
        let (sort_indices, filtered): (Vec<usize>, Vec<usize>) = filtered.into_iter().unzip();
        self.filtered_indices = filtered;
        let mut content = String::with_capacity(content_size);
        let absolute_prefix = if matches!(
            options
                .display_type
                .selected
                .expect("There must be a display type"),
            DisplayType::AbsolutePath
        ) {
            self.open_folder
                .as_ref()
                .expect("A folder must have been opened to get here")
                .to_owned()
        } else {
            String::new()
        };
        sort_indices.into_iter().for_each(|i| {
            content.push_str(&absolute_prefix);
            content.push_str(entries[i]);
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
            DisplayType::JustName => &entry.path[entry.last_sep + 1..],
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
