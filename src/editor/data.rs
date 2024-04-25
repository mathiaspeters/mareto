use std::path::PathBuf;

use iced::widget::text_editor;

use crate::fs::FileSystemEntry;

#[derive(Debug, Default)]
pub struct EditorState {
    pub open_folder: Option<PathBuf>,
    pub entries: Vec<FileSystemEntry>,

    pub contents: text_editor::Content,
    pub find_and_replace: FindAndReplace,
}

impl EditorState {
    pub fn show_entries(&mut self) {
        let content = self
            .entries
            .iter()
            .map(|e| e.path.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        self.contents = text_editor::Content::with_text(&content);
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
