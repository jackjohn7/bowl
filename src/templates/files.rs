use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct FileContent {
    /// File path relative to the caller
    pub file_path: String,
    /// The inescaped content of the file
    pub content: Vec<u8>,
}

impl FileContent {
    pub fn from_path(path: PathBuf) -> Result<Self, String> {
        let file_path = path.clone().to_str().unwrap().to_owned();
        let content = fs::read(path).map_err(|e| e.to_string())?;
        Ok(Self { file_path, content })
    }
}
