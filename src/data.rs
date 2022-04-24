use std::{path::Path, collections::HashMap, io::Read};
use file_lock::{FileLock, FileOptions};
use serde::{Serialize, Deserialize};

/// Manages oprovides on-disk data for the rest of the application.
/// Only one JsonDataService should be instantiated per file.
pub struct JsonDataService {
    json_file: FileLock,
    data: JayData,
}

impl JsonDataService {
    pub fn new<P: AsRef<Path>>(path: P, is_blocking: bool) -> Result<Self, Error> {
        let file_options = FileOptions::new()
            .write(true)
            .create(true);
        
        let mut file_lock = FileLock::lock(path, is_blocking, file_options)?;
        let mut json_string: String = String::new();
        file_lock.file.read_to_string(&mut json_string)?;
        let data: JayData = json5::from_str(&json_string)?;

        Ok(JsonDataService{
            json_file: file_lock,
            data: data
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JayData {
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>
}

pub enum Error {
    IoError(std::io::Error),
    Json5Error(json5::Error)
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<json5::Error> for Error {
    fn from(e: json5::Error) -> Self {
        Self::Json5Error(e)
    }
}