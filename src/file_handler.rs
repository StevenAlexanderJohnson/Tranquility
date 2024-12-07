use std::{
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use actix_multipart::form::tempfile::TempFile;

#[derive(Clone)]
pub struct LocalFileHandler {
    storage_folder: String,
}

impl LocalFileHandler {
    pub fn new() -> Self {
        Self {
            storage_folder: std::env::var("STORAGE_FOLDER").expect("STORAGE_FOLDER is not set"),
        }
    }

    pub fn check_destination(&self) -> Result<(), Box<dyn std::error::Error>> {
        if Path::new(&self.storage_folder).exists() {
            return Ok(());
        }
        std::fs::create_dir(&self.storage_folder)?;
        Ok(())
    }

    pub fn store_file(
        &self,
        file: &TempFile,
        file_name: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Unable to get duration since epoch");
        let file_name = format!(
            "{}-{}",
            timestamp.as_millis(),
            sanitize_filename::sanitize(file_name)
        );

        let mut file_path = PathBuf::from(&self.storage_folder);
        file_path.push(&file_name);

        let file_path_string = file_path
            .to_str()
            .ok_or("Error creating string from file path.")?;

        println!("TempFile: {:?}", file.file.path());
        println!("File Path: {:?}", file_path);
        match std::fs::copy(file.file.path(), &file_path) {
            Ok(_) => Ok((file_name, file_path_string.into())),
            Err(e) => {
                println!("Error renaming file: {:?}", e);
                Err(e.into())
            }
        }
        // Ok((file_name, file_path_string.into()))
    }

    pub fn delete_file(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file_path = PathBuf::from(&self.storage_folder);
        file_path.push(file_name);

        std::fs::remove_file(file_path)?;

        Ok(())
    }
}
