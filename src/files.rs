use std::fs::{self, DirEntry};

pub fn file_entries(entry: DirEntry) -> Result<Vec<DirEntry>, String> {
    if entry.metadata().unwrap().is_dir() {
        Ok(fs::read_dir(entry.path())
            .map_err(|e| e.to_string())?
            .map(|x| x.map_err(|e| e.to_string()))
            .collect::<Result<Vec<DirEntry>, String>>()?
            .into_iter()
            .flat_map(file_entries)
            .flatten()
            .collect::<Vec<DirEntry>>())
    } else {
        Ok(vec![entry])
    }
}

