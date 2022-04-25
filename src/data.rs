use std::{path::Path, collections::HashMap, io::Read};
use chrono::{Utc, DateTime};
use file_lock::{FileLock, FileOptions};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

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

    pub fn select_container_by_uuid(&self, uuid: Uuid) -> Result<&Container, Error> {
        for c in &self.data.containers {
            if c.uuid == uuid {
                return Ok(c);
            }
        }

        Err(Error::UuidNotFound(uuid))
    }

    pub fn select_item_by_uuid(&self, uuid: Uuid) -> Result<&Container, Error> {
        for c in &self.data.containers {
            if c.uuid == uuid {
                return Ok(c);
            }
        }

        Err(Error::UuidNotFound(uuid))
    }

    pub fn insert_container(&mut self, container: Container) -> () {
        self.data.history.push(History::Created {
            record_uuid: container.uuid,
            datetime: Utc::now()
        });

        self.data.containers.push(container);
    }

    pub fn insert_item(&mut self, item: Item) -> () {
        self.data.history.push(History::Created {
            record_uuid: item.uuid,
            datetime: Utc::now()
        });

        self.data.items.push(item);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JayData {
    containers: Vec<Container>,
    items: Vec<Item>,
    history: Vec<History>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    uuid: Uuid,
    name: String,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    uuid: Uuid,
    name: String,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum History {
    Created {
        record_uuid: Uuid,
        datetime: DateTime<Utc>,
    },
    Modified {
        record_uuid: Uuid,
        datetime: DateTime<Utc>,
        deltas: HashMap<String, serde_json::Value>,
    },
    Deleted {
        record_uuid: Uuid,
        datetime: DateTime<Utc>
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    UuidNotFound(Uuid),
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