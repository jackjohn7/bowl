use directories::ProjectDirs;
use std::fs::{self, DirEntry, File};
use std::io::Write;

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

pub fn save_file_locally(filename: String, content: Vec<u8>) -> Result<(), String> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "jackjohn7", "bowl") {
        let mut p = proj_dirs.data_dir().to_path_buf();
        fs::create_dir_all(&p).map_err(|e| format!("Failed to make directory: {}", e))?;
        p.push(filename);
        File::create(p)
            .map_err(|e| format!("Error creating bowlfile: {}", e))?
            .write_all(&content)
            .map_err(|e| format!("Failed to write bowlfile: {}", e))?;
        Ok(())
    } else {
        Err("Failed to locate data directory".into())
    }
}

pub fn get_file_locally(template: String) -> Result<Vec<u8>, String> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "jackjohn7", "bowl") {
        let mut p = proj_dirs.data_dir().to_path_buf();
        fs::create_dir_all(&p).map_err(|e| format!("Failed to make directory: {}", e))?;
        p.push(&template);
        p.set_extension("bowl");
        dbg!(&p);
        let content = fs::read(p).map_err(|e| format!("Failed to read bowlfile: {}", e))?;
        Ok(content)
    } else {
        Err("Failed to locate data directory".into())
    }
}
