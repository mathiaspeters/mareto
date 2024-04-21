use std::path::PathBuf;

use iced::widget::text_editor;

#[derive(Debug, Default)]
pub struct EditorState {
    pub contents: text_editor::Content,
    pub open_folder: Option<PathBuf>,
    pub find_and_replace: FindAndReplace,
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
