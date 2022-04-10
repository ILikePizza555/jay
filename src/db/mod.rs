use std::{path::Path, fmt::Debug};
use chrono::{DateTime, Utc, NaiveDateTime};
use rusqlite::{Connection, Row, named_params};
use uuid::Uuid;

mod error;

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

    pub fn from_row_offset(row: &Row, index_offset: usize) -> error::Result<Self> {
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
    pub fn new(name: String, description: Option<String>, r_type: Option<String>) -> Self {
        ContainerRow {
            uuid: Uuid::new_v4(),
            name,
            description,
            r_type,
            created_date: Utc::now()
        }
    }

    pub fn from_row(row: &Row) -> error::Result<Self> {
        Self::from_row_offset(row, 0)
    }

    pub fn from_row_offset(row: &Row, index_offset: usize) -> error::Result<Self> {
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

/// Wrapper around the rusqlite Connection object to provide our own query methods on.
pub struct DatabaseConnection (Connection);

impl DatabaseConnection {
    pub fn open<P : AsRef<Path>>(p: P) -> core::result::Result<DatabaseConnection, rusqlite::Error> {
        Connection::open(p).map(|c| DatabaseConnection(c))
    }

    /// Directly inserts a new row into the container table.
    pub fn insert_container(&self, container: ContainerRow) -> error::Result<()> {
        let mut statement = self.0.prepare(
        "INSERT INTO 'containers' (uuid, name, description, type, created_date)
            VALUES (:uuid, :name, :description, :type, :created_date)"
        )?;

        let result= statement.execute(named_params! {
            ":uuid":           container.uuid.to_hyphenated().to_string(),
            ":name":           container.name,
            ":description":    container.description,
            ":type":           container.r_type,
            ":created_date":   container.created_date.timestamp()
        })?;

        assert!(result == 1);

        Ok(())
    }

    /// Directly inserts a new item into the items table.
    pub fn insert_item(&self, item: ItemRow) -> error::Result<()> {
        let mut statement = self.0.prepare(
        "INSERT INTO 'items' (uuid, name, description, type, quantity, created_date, modified_date, status)
            VALUES (:uuid, :name, :description, :type, :quantity, :created_date, :modified_date, :status)"
        )?;

        let result = statement.execute(named_params! {
            ":uuid":           item.uuid.to_hyphenated().to_string(),
            ":name":           item.name,
            ":description":    item.description,
            ":type":           item.r_type,
            ":quantity":       item.quantity,
            ":created_date":   item.modified_date.timestamp(),
            ":modified_date":  item.modified_date.timestamp(),
            ":status":         item.status
        })?;

        assert!(result == 1);

        Ok(())
    }

    /// Selects container(s) from the database with the specified name
    pub fn select_container_by_name(&self, name: &str) -> error::Result<Vec<ContainerRow>> {
        let mut statement = self.0.prepare(
            "SELECT uuid, name, description, type, created_date FROM containers WHERE name = ?1"
        )?;

        let r: error::Result<Vec<ContainerRow>> = statement
            .query_and_then([name], |r| ContainerRow::from_row_offset(r, 0))
            ?.collect();
        
        r
    }

    /// Selects the container from the database with the given uuid.
    pub fn select_container_by_uuid(&self, uuid: &Uuid) -> error::Result<ContainerRow> {
        let mut statement = self.0.prepare(
            "SELECT uuid, name, description, type, created_date FROM containers WHERE uuid = ?1"
        )?;

        let uuid_str = uuid.to_hyphenated().to_string();
        let mut rows = statement.query([uuid_str])?;
        
        match rows.next()? {
            Some(e) => Ok(ContainerRow::from_row(e)?),
            None => Err(error::DatabaseError::ContainerUuidNotFoundError(*uuid))
        }
    }

    /// Determines if the provided string is a uuid or name, then returns the containers which matches the provided identifier.
    pub fn find_container_by_name_or_uuid(&self, name_or_uuid: &str) -> error::Result<ContainerRow> {
        Uuid::parse_str(name_or_uuid)
            .map_err(|e| error::DatabaseError::UuidError(e))
            .and_then(|uuid| self.select_container_by_uuid(&uuid))
            .or_else(|_| {
                let mut containers = self.select_container_by_name(name_or_uuid)?;
                if containers.len() > 1 {
                    Err(error::DatabaseError::ContainerAmbigiousNameError(name_or_uuid.to_string(), containers))
                } else {
                    Ok(containers.remove(0))
                }
            })
    }
}