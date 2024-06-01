use regex::{Regex, RegexBuilder};

use crate::{
    bit_set::BitSet,
    fs::{EntryType, FileSystemEntry},
};

use super::EditorState;

#[derive(Debug, Clone)]
pub struct FilterOptions {
    pub filter_input: FilterState<FilterInput>,
    pub min_depth: FilterState<DepthLimit>,
    pub max_depth: FilterState<DepthLimit>,
    pub show_files: FilterState<bool>,
    pub show_folders: FilterState<bool>,
}

impl FilterOptions {
    pub fn get_visibility_vectors(&self) -> Vec<&BitSet> {
        vec![
            &self.filter_input.is_visible,
            &self.min_depth.is_visible,
            &self.max_depth.is_visible,
            &self.show_files.is_visible,
            &self.show_folders.is_visible,
        ]
    }

    pub fn resize_filters(&mut self, new_size: usize) {
        self.filter_input.is_visible.resize(new_size);
        self.min_depth.is_visible.resize(new_size);
        self.max_depth.is_visible.resize(new_size);
        self.show_files.is_visible.resize(new_size);
        self.show_folders.is_visible.resize(new_size);
    }

    pub fn update_text_filter(&mut self, editor_state: &EditorState) {
        self.filter_input.state.update_regex();
        self.apply_text_filter(editor_state);
    }

    pub fn normalize_min_depth(&mut self) {
        match (self.min_depth.state.limit, self.max_depth.state.limit) {
            (Some(min_limit), Some(max_limit)) if min_limit > max_limit => {
                self.min_depth.state.limit = Some(max_limit);
            }
            _ => {}
        }
    }

    pub fn normalize_max_depth(&mut self) {
        match (self.min_depth.state.limit, self.max_depth.state.limit) {
            (Some(min_limit), Some(max_limit)) if max_limit < min_limit => {
                self.max_depth.state.limit = Some(min_limit);
            }
            _ => {}
        }
    }

    pub fn update_min_depth(&mut self, editor_state: &EditorState) {
        let is_active = self.min_depth.state.is_active;
        let limit = self.min_depth.state.limit.unwrap_or(0);
        self.min_depth
            .update(editor_state, |entry| !is_active || entry.depth >= limit);
    }

    pub fn update_max_depth(&mut self, editor_state: &EditorState) {
        let is_active = self.max_depth.state.is_active;
        let limit = self.max_depth.state.limit.unwrap_or(usize::MAX);
        self.max_depth
            .update(editor_state, |entry| !is_active || entry.depth <= limit);
    }

    pub fn update_show_files(&mut self, editor_state: &EditorState) {
        let should_show_files = self.show_files.state;
        self.show_files.update(editor_state, |entry| {
            should_show_files || !matches!(entry.entry_type, EntryType::File)
        });
    }

    pub fn update_show_folders(&mut self, editor_state: &EditorState) {
        let should_show_folders = self.show_folders.state;
        self.show_folders.update(editor_state, |entry| {
            should_show_folders || !matches!(entry.entry_type, EntryType::Folder)
        });
    }

    fn apply_text_filter(&mut self, editor_state: &EditorState) {
        let re = match &self.filter_input.state.regex {
            Some(Ok(re)) => Some(re),
            Some(Err((re, _))) => re.as_ref(),
            _ => None,
        };

        self.filter_input
            .is_visible
            .resize(editor_state.entries.len());
        (0..self.filter_input.is_visible.size)
            .zip(editor_state.entries.iter())
            .for_each(|(i, entry)| {
                let is_set = match &re {
                    Some(re) if re.is_match(&entry.path) => true,
                    None => {
                        if self.filter_input.state.case_sensitive {
                            entry.path.contains(&self.filter_input.state.input)
                        } else {
                            entry
                                .path
                                .to_lowercase()
                                .contains(&self.filter_input.state.input.to_lowercase())
                        }
                    }
                    _ => false,
                };
                self.filter_input.is_visible.set_bit(i, is_set);
            });
    }
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self {
            filter_input: FilterState::new(Default::default()),
            min_depth: FilterState::new(Default::default()),
            max_depth: FilterState::new(Default::default()),
            show_files: FilterState::new(true),
            show_folders: FilterState::new(true),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterState<T> {
    pub state: T,
    pub is_visible: BitSet,
}

impl<T> FilterState<T> {
    pub fn new(initial: T) -> Self {
        Self {
            state: initial,
            is_visible: BitSet::new(),
        }
    }

    pub fn update<F>(&mut self, editor_state: &EditorState, op: F)
    where
        F: Fn(&FileSystemEntry) -> bool,
    {
        self.is_visible.resize(editor_state.entries.len());
        (0..self.is_visible.size)
            .zip(editor_state.entries.iter())
            .for_each(|(i, entry)| {
                self.is_visible.set_bit(i, op(entry));
            });
    }
}

#[derive(Debug, Clone, Default)]
pub struct FilterInput {
    pub input: String,
    pub regex: Option<Result<Regex, (Option<Regex>, String)>>,
    pub use_regex: bool,
    pub case_sensitive: bool,
}

impl FilterInput {
    pub fn update_regex(&mut self) {
        match (self.input.is_empty(), self.use_regex) {
            (false, true) => {
                self.regex = match RegexBuilder::new(self.input.as_str())
                    .case_insensitive(!self.case_sensitive)
                    .build()
                {
                    Ok(re) => Some(Ok(re)),
                    Err(err) => {
                        let mut previous_regex = None;
                        std::mem::swap(&mut previous_regex, &mut self.regex);
                        let previous_regex = match previous_regex {
                            Some(Ok(re)) => Some(re),
                            Some(Err((re, _))) => re,
                            _ => None,
                        };
                        let error_message = match err {
                            regex::Error::Syntax(s) => s,
                            regex::Error::CompiledTooBig(_) => "Regex too big".to_owned(),
                            _ => unimplemented!(),
                        };
                        Some(Err((previous_regex, error_message)))
                    }
                }
            }
            _ => self.regex = None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DepthLimit {
    pub is_active: bool,
    pub limit: Option<usize>,
}

impl DepthLimit {
    pub fn set_from_str(&mut self, mut limit: String) {
        limit.retain(|c| c.is_numeric());
        self.limit = if limit.is_empty() {
            None
        } else {
            Some(limit.parse().expect("Only numbers should still be there"))
        }
    }
}
