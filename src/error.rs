use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    UuidError(uuid::Error),
    UuidNotFoundError(Uuid),
    NameNotFoundError(String),
    AmbigiousNameError(String),
    UuidRequiredError()
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

impl From<uuid::Error> for Error {
    fn from(e: uuid::Error) -> Self {
        Self::UuidError(e)
    }
}