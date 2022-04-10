use super::error::Result;
use std::fmt::Debug;
use chrono::{DateTime, Utc, NaiveDateTime};
use rusqlite::Row;
use uuid::Uuid;

/// DTO object used to hold a row of data from the 'items' table.
#[derive(Debug)]
pub struct ItemRow {
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub r_type: Option<String>,
    pub quantity: usize,
    pub created_date: DateTime<Utc>,
    pub modified_date: DateTime<Utc>,
    pub status: String
}

impl ItemRow {
    pub fn new(name: String, description: Option<String>, r_type: Option<String>, quantity: usize, status: String) -> Self {
        let now = Utc::now();
        
        ItemRow { 
            uuid: Uuid::new_v4(),
            name,
            description,
            r_type,
            quantity,
            created_date: now,
            modified_date: now, 
            status
        }
    }

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
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub r_type: Option<String>,
    pub created_date: DateTime<Utc>,
}

impl ContainerRow {
    pub fn new(name: String, description: Option<String>, r_type: Option<String>) -> Self {
        ContainerRow {
            uuid: Uuid::new_v4(),
            name,
            description,
            r_type,
            created_date: Utc::now()
        }
    }

    pub fn from_row(row: &Row) -> Result<Self> {
        Self::from_row_offset(row, 0)
    }

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
