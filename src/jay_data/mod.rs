pub mod models;

use crate::error::Error;
use std::{path::Path, io::{Read, Seek, SeekFrom}, str::FromStr};
use file_lock::{FileLock, FileOptions};
use uuid::Uuid;

use self::models::{JayDataModel, ContainerModel, ItemModel};

/// Manages oprovides on-disk data for the rest of the application.
/// Only one JsonDataService should be instantiated per file.
pub struct JsonDataService {
    json_file: FileLock,
    pub models: JayDataModel,
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

        let models: JayDataModel = if json_string.is_empty() {
            JayDataModel::default()
        } else {
            serde_json::from_str(&json_string)?
        };

        Ok(JsonDataService{
            json_file: file_lock,
            models,
        })
    }

    pub fn flush(&self) -> Result<(), Error> {
        // Truncate the file before writing to it. Not doing this results in invalid json or null bytes or other bad things.
        let mut file = &self.json_file.file;
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        
        serde_json::to_writer(file, &self.models)
            .map_err(|e| -> Error {
                e.into()
            })
    }

    pub fn find_container_by_uuid(&self, uuid: Uuid) -> Result<&ContainerModel, Error> {
        self.models.containers 
            .iter()
            .find(|&c| c.uuid == uuid)
            .ok_or(Error::UuidNotFoundError(uuid))
    }

    pub fn find_container_by_uuid_str(&self, uuid_str: &str) -> Result<&ContainerModel, Error> {
        let uuid = Uuid::from_str(uuid_str)?;
        self.find_container_by_uuid(uuid)
    }

    pub fn find_item_by_uuid(&self, uuid: Uuid) -> Result<&ItemModel, Error> {
        self.models.items
            .iter()
            .find(|&i| i.uuid == uuid)
            .ok_or(Error::UuidNotFoundError(uuid))
    }

    pub fn find_item_by_uuid_str(&self, uuid_str: &str) -> Result<&ItemModel, Error> {
        let uuid = Uuid::from_str(uuid_str)?;
        self.find_item_by_uuid(uuid)
    }
}