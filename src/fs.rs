use std::path::Path;

use crate::mareto::Error;

#[derive(Debug, Copy, Clone)]
pub enum EntryType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct FileSystemEntry {
    pub path: String,
    pub depth: usize,
    pub entry_type: EntryType,
}

pub fn get_entries_for_path(path: impl AsRef<Path>) -> Result<Vec<FileSystemEntry>, Error> {
    let mut entries = vec![];
    get_entries_for_path_impl(path, 1, &mut entries)?;
    Ok(entries)
}

fn get_entries_for_path_impl(
    path: impl AsRef<Path>,
    depth: usize,
    entries: &mut Vec<FileSystemEntry>,
) -> Result<(), Error> {
    for de in std::fs::read_dir(path)? {
        let de = de?;
        if let Ok(file_type) = de.file_type() {
            if let Some(path) = de.path().to_str() {
                let path = path.to_owned();
                let ft = if file_type.is_dir() {
                    get_entries_for_path_impl(&path, depth + 1, entries)?;
                    Some(EntryType::Folder)
                } else if file_type.is_file() {
                    Some(EntryType::File)
                } else {
                    None
                };
                if let Some(entry_type) = ft {
                    let entry = FileSystemEntry {
                        path,
                        depth,
                        entry_type,
                    };
                    entries.push(entry);
                }
            }
        }
    }
    Ok(())
}
