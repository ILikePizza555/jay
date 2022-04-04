use core::fmt;
use std::{path::Path, error::Error, fmt::{Display, Debug}};
use chrono::{DateTime, Utc, NaiveDateTime};
use rusqlite::{Connection, Row, Error as RusqliteError};
use uuid::{Uuid, Error as UuidError};

#[derive(Debug)]
pub enum DatabaseError {
    UuidError(UuidError),
    SqliteError(RusqliteError)
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            DatabaseError::UuidError(e) => fmt::Display::fmt(&e, f),
            DatabaseError::SqliteError(e) => fmt::Display::fmt(&e, f)
        }
    }
}

impl Error for DatabaseError {

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

type Result<T> = core::result::Result<T, DatabaseError>;

/// DTO object used to hold a row of data from the 'items' table.
#[derive(Debug)]
pub struct ItemRow {
    uuid: Uuid,
    name: String,
    description: Option<String>,
    r_type: Option<String>,
    quantity: usize,
    created_date: DateTime<Utc>,
    modified_date: DateTime<Utc>,
    status: String
}

impl ItemRow {
    pub fn from_row_offset(row: &Row, index_offset: usize) -> Result<Self> {
        Ok(ItemRow {
            uuid: Uuid::parse_str(row.get::<usize, String>(index_offset)?.as_str())?,
            name: row.get(index_offset + 1)?,
            description: row.get(index_offset + 2)?,
            r_type: row.get(index_offset + 3)?,
            quantity: row.get(index_offset + 4)?,
            created_date: DateTime::from_utc(
        NaiveDateTime::from_timestamp(row.get(index_offset + 5)?, 0),Utc),
            modified_date: DateTime::from_utc(
        NaiveDateTime::from_timestamp(row.get(index_offset + 6)?, 0), Utc),
            status: row.get(index_offset + 7)?
        })
    }
}

/// DTO object used to hold a row a data from the 'containers' table.
#[derive(Debug)]
pub struct ContainerRow {
    uuid: Uuid,
    name: String,
    description: Option<String>,
    r_type: Option<String>,
    created_date: DateTime<Utc>,
}

impl ContainerRow {
    pub fn from_row_offset(row: &Row, index_offset: usize) -> Result<Self> {
        Ok(ContainerRow {
            uuid: Uuid::parse_str(row.get::<usize, String>(index_offset)?.as_str())?,
            name: row.get(index_offset + 1)?,
            description: row.get(index_offset + 2)?,
            r_type: row.get(index_offset + 3)?,
            created_date: DateTime::from_utc(
                NaiveDateTime::from_timestamp(row.get(index_offset + 4)?, 0),Utc),
        })
    }
}

/// Used to hold the result when querying both the 'items' and 'containers' tables.
pub enum ItemOrContainerRow {
    Item (ItemRow),
    Container (ContainerRow)
}

/// Wrapper around the rusqlite Connection object to provide our own query methods on.
pub struct DatabaseConnection (Connection);

impl DatabaseConnection {
    pub fn open<P : AsRef<Path>>(p: P) -> core::result::Result<DatabaseConnection, rusqlite::Error> {
        Connection::open(p).map(|c| DatabaseConnection(c))
    }

    pub fn select_all_items_and_containers(&self) -> Result<Vec<ItemOrContainerRow>> {
        let mut statement = self.0.prepare(
        r#"SELECT 'item' as object_type, uuid, name, description, type, created_date FROM items
               UNION
               SELECT 'container' as object_type, uuid, name, description, type, created_date FROM containers;"#)?;

        // Yeah, I'm returning a vector instead of an interator or something
        // I know it's not the "idiomatic" way of doing things in Rust,
        // but fucking rusqlite is doing some insane bullshit with lifetimes and the way it's query's functions work
        // and I am *tired* of dealing with the god-damn borrow checker.
        let r: Result<Vec<ItemOrContainerRow>> = statement.query_and_then([], |row| {
            let object_type: String = row.get(0)?;

            if object_type.eq_ignore_ascii_case("item") {
                Ok(ItemOrContainerRow::Item(ItemRow::from_row_offset(row, 1)?))
            } else if object_type.eq_ignore_ascii_case("container") {
                Ok(ItemOrContainerRow::Container(ContainerRow::from_row_offset(row, 1)?))
            } else {
                panic!("object type was not item or container")
            }
        })?.collect();

        r
    }
}