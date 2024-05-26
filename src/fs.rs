use std::path::Path;

use crate::mareto::Error;

#[derive(Debug, Copy, Clone)]
pub enum EntryType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct FileSystemEntry {
    pub og_path: String,
    pub path: String,
    pub depth: usize,
    pub entry_type: EntryType,
}

pub fn get_entries_for_path(initial_path: &str) -> Result<Vec<FileSystemEntry>, Error> {
    let mut path_finder = PathFinder::new(initial_path);
    path_finder.get_entries_for_path(initial_path, 1)?;
    Ok(path_finder.entries())
}

struct PathFinder<'a> {
    initial_path: &'a str,
    entries: Vec<FileSystemEntry>,
}

impl<'a> PathFinder<'a> {
    fn new(initial_path: &'a str) -> Self {
        Self {
            initial_path,
            entries: Vec::with_capacity(50),
        }
    }

    fn entries(self) -> Vec<FileSystemEntry> {
        self.entries
    }

    fn get_entries_for_path(&mut self, path: impl AsRef<Path>, depth: usize) -> Result<(), Error> {
        for de in std::fs::read_dir(path)? {
            let de = de?;
            if let Ok(file_type) = de.file_type() {
                if let Some(path) = de.path().to_str() {
                    let path = path.to_owned();
                    let ft = if file_type.is_dir() {
                        self.get_entries_for_path(&path, depth + 1)?;
                        Some(EntryType::Folder)
                    } else if file_type.is_file() {
                        Some(EntryType::File)
                    } else {
                        None
                    };
                    if let Some(entry_type) = ft {
                        let og_path = path
                            .strip_prefix(self.initial_path)
                            .expect("All paths found need to be under the initial path")
                            .to_owned();
                        let path = og_path.clone();
                        let entry = FileSystemEntry {
                            og_path,
                            path,
                            depth,
                            entry_type,
                        };
                        self.entries.push(entry);
                    }
                }
            }
        }
        Ok(())
    }
}
