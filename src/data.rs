use std::{path::Path, collections::HashMap, io::Read};
use chrono::{Utc, DateTime};
use file_lock::{FileLock, FileOptions};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Manages oprovides on-disk data for the rest of the application.
/// Only one JsonDataService should be instantiated per file.
pub struct JsonDataService {
    json_file: FileLock,
    pub data: JayData,
}

impl JsonDataService {
    pub fn new<P: AsRef<Path>>(path: P, is_blocking: bool) -> Result<Self, Error> {
        let file_options = FileOptions::new()
            .write(true)
            .create(true);
        
        let mut file_lock = FileLock::lock(path, is_blocking, file_options)?;
        let mut json_string: String = String::new();
        file_lock.file.read_to_string(&mut json_string)?;
        let data: JayData = serde_json::from_str(&json_string)?;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct JayData {
    pub containers: Vec<ContainerModel>,
    pub items: Vec<ItemModel>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerModel {
    pub uuid: Uuid,
    pub name: String,
    pub created_date: DateTime<Utc>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>
}

impl ContainerModel {
    pub fn new(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name,
            created_date: Utc::now(),
            extra: HashMap::new()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemModel {
    pub uuid: Uuid,
    pub name: String,
    pub created_date: DateTime<Utc>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>
}

impl ItemModel {
    pub fn new(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name,
            created_date: Utc::now(),
            extra: HashMap::new()
        }
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