mod models;

use std::{path::Path, io::Read};
use file_lock::{FileLock, FileOptions};

use self::models::JayDataModel;

/// Manages oprovides on-disk data for the rest of the application.
/// Only one JsonDataService should be instantiated per file.
pub struct JsonDataService {
    json_file: FileLock,
    data: JayDataModel,
}

impl JsonDataService {
    pub fn new<P: AsRef<Path>>(path: P, is_blocking: bool) -> Result<Self, Error> {
        let file_options = FileOptions::new()
            .read(true)
            .write(true)
            .create(true);
        
        let mut file_lock = FileLock::lock(path, is_blocking, file_options)?;
        let mut json_string: String = String::new();
        file_lock.file.read_to_string(&mut json_string)?;


        let data: JayDataModel = if json_string.is_empty() {
            JayDataModel::default()
        } else {
            serde_json::from_str(&json_string)?
        };

        Ok(JsonDataService{
            json_file: file_lock,
            data: data,
        })
    }

    pub fn flush(&self) -> Result<(), Error> {
        serde_json::to_writer(&self.json_file.file, &self.data)
            .map_err(|e| -> Error {
                e.into()
            })
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::JsonError(e)
    }
}