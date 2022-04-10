use core::fmt;
use uuid::{Uuid, Error as UuidError};
use rusqlite::Error as RusqliteError;
use super::dto::ContainerRow;

#[derive(Debug)]
pub enum DatabaseError {
    ContainerUuidNotFoundError(Uuid),
    ContainerAmbigiousNameError(String, Vec<ContainerRow>),

    UuidError(UuidError),
    SqliteError(RusqliteError)
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            DatabaseError::ContainerUuidNotFoundError(uuid) => 
                write!(f, "Container with uuid {} not found.", uuid.to_hyphenated().to_string()),

            DatabaseError::ContainerAmbigiousNameError(name, matches) =>
                write!(f, "Identifier {} is ambigious, found {} containers with that name.", name, matches.len()),

            DatabaseError::UuidError(e) => fmt::Display::fmt(&e, f),
            DatabaseError::SqliteError(e) => fmt::Display::fmt(&e, f)
        }
    }
}

impl From<UuidError> for DatabaseError {
    fn from(e: UuidError) -> Self {
        DatabaseError::UuidError(e)
    }
}

impl From<RusqliteError> for DatabaseError {
    fn from(e: RusqliteError) -> Self {
        DatabaseError::SqliteError(e)
    }
}

pub type Result<T> = core::result::Result<T, DatabaseError>;