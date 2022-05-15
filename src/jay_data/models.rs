use std::collections::HashMap;
use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use tabled::Tabled;
use uuid::Uuid;

use crate::useful::{format_table_uuid, format_table_option_display, format_table_option};

/// The root model. Holds all collections.
#[derive(Debug, Serialize, Deserialize)]
pub struct JayDataModel {
    pub containers: Vec<ContainerModel>,
    pub items: Vec<ItemModel>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>
}

impl Default for JayDataModel {
    fn default() -> Self {
        Self { 
            containers: Vec::new(),
            items: Vec::new(),
            extra: HashMap::new()
        }
    }
}

/// Model that describes a Container
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerModel {
    pub uuid: Uuid,
    pub created_date: DateTime<Utc>,

    pub name: String,
    pub description: Option<String>,
    pub location: Option<Uuid>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>
}

impl ContainerModel {
    pub fn new(name: String, description: Option<String>, location: Option<Uuid>, extra: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            created_date: Utc::now(),
            name,
            description,
            location,
            extra: extra.unwrap_or_default()
        }
    }
}

impl Tabled for ContainerModel {
    const LENGTH: usize = 48;

    fn fields(&self) -> Vec<String> {
        vec! [
            format_table_uuid(&self.uuid),
            self.created_date.to_rfc2822(),
            self.name.clone(),
            format_table_option_display(&self.description),
            format_table_option(&self.location, |uuid| format_table_uuid(uuid))
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            String::from("UUID"),
            String::from("Created Date"),
            String::from("Name"),
            String::from("Description"),
            String::from("Location"),
        ]
    }
}

/// Model that describes an Item
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemModel {
    pub uuid: Uuid,
    pub created_date: DateTime<Utc>,

    pub name: String,
    pub location: Uuid,
    pub quantity: u64,
    pub description: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>
}

impl ItemModel {
    pub fn new(name: String, description: Option<String>, location: Uuid, quantity: u64, extra: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            created_date: Utc::now(),
            name,
            location,
            quantity,
            description,
            extra: extra.unwrap_or_default()
        }
    }
}