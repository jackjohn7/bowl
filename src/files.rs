use std::fs::{self, DirEntry};

pub fn file_entries(entry: DirEntry) -> Vec<DirEntry> {
    if entry.metadata().unwrap().is_dir() {
        fs::read_dir(entry.path())
            .map_err(|e| e.to_string())
            .unwrap()
            .map(|x| x.map_err(|e| e.to_string()))
            .collect::<Result<Vec<DirEntry>, String>>()
            .unwrap()
            .into_iter()
            .map(file_entries)
            .flatten()
            .collect::<Vec<DirEntry>>()
    } else {
        vec![entry]
    }
}

