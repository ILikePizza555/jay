mod jay_data;
mod error;

use std::collections::HashMap;

use clap::{Parser, Subcommand};
use error::Error;
use jay_data::{JsonDataService, models::{ContainerModel, ItemModel}};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: ActionCommands
}

#[derive(Subcommand)]
enum ActionCommands {
    /// Create and insert a new object into the catalogue. 
    #[clap(subcommand)]
    Add (AddCommands),

    /// List objects in the catalogue.
    #[clap(subcommand)]
    List (ListCommands),

    /// Remove an object from the catalogue.
    Delete { name_or_id: String }
}

#[derive(Subcommand)]
enum AddCommands {
    Item {
        name: String,
        location: String,

        #[clap(default_value_t = 1)]
        quantity: u64,

        #[clap(short, long)]
        description: Option<String>,

        #[clap(name = "type", short, long)]
        r_type: Option<String>
    },

    Container {
        /// The name of the container.
        name: String,
        location: Option<String>,

        #[clap(short, long)]
        description: Option<String>,

        #[clap(name = "type", short, long)]
        r_type: Option<String>
    }
}

impl AddCommands {
    fn get_location(&self) -> Option<&str> {
        match self {
            Self::Item {location, ..} => Some(location),
            Self::Container {location, ..} => location.as_ref()
        }.map(|s| s.as_str())
    }

    fn get_type(&self) -> Option<&str> {
        match self {
            Self::Item {r_type, ..} => r_type.as_ref(),
            Self::Container {r_type, ..} => r_type.as_ref()
        }.map(|s| s.as_str()) 
    }
}

#[derive(Subcommand)]
enum ListCommands {
    /// Lists all objects in the database.
    All,
    /// Lists all containers with the specified name or id. If not specified, lists all containers.
    Container { name_or_id: Option<String> },
    /// Lists all items with the specified name or id
    Item { name_or_id: Option<String> },
    /// Lists all objects inside the specified container.
    Within { container_name_or_id: Option<String> }
}

fn main() {
    let cli = Cli::parse();
    let mut service = JsonDataService::new("jay.json", true).expect("Error reading jay.json.");

    match cli.command {
        ActionCommands::Add(add_cmd) => add_object(&mut service, add_cmd).expect("Error creating new object."),
        _ => println!("Not implemented")
    }

    service.flush().expect("Error writing to jay.json.");
}

fn add_object(service: &mut JsonDataService, add_cmd: AddCommands) -> Result<(), Error> {
    let location_uuid = add_cmd.get_location().map(
        |l| get_container_by_location(&service, l)
            .map(|c| c.uuid)
    )
        .transpose()?;
    
    let type_value = serde_json::to_value(add_cmd.get_type())?;
    let extras = HashMap::from([
        ("type".to_string(), type_value)
    ]);

    match add_cmd {
        AddCommands::Item { name, location: _, quantity, description, r_type: _ } => {
            let uuid = location_uuid.ok_or(Error::UuidRequiredError())?;
            let item_model = ItemModel::new(name, description, uuid, quantity, Some(extras));

            Ok(service.models.items.push(item_model))
        }
        AddCommands::Container {name, location: _, description, r_type: _} => {
            let container_model = ContainerModel::new(name, description, location_uuid, Some(extras));
            Ok(service.models.containers.push(container_model))
        }
    }
}

/// Checks if the provided location is a uuid. 
/// If it is, then searches containers for a match. 
/// Otherwise searches containers for one with the same name as location.
/// If multiple are found, an error is returned. 
fn get_container_by_location<'l>(service: &'l JsonDataService, location: &str) -> Result<&'l ContainerModel, Error> {
    match <uuid::Uuid as std::str::FromStr>::from_str(location) {
        Err(_) => {
            let name_matches: Vec<&ContainerModel> = service.models.containers.iter()
                .filter(|&c| c.name == location)
                .collect();
            
            if name_matches.len() == 1 {
                Ok(name_matches[0])
            } else if name_matches.is_empty() {
                Err(Error::NameNotFoundError(location.to_string()))
            } else {
                Err(Error::AmbigiousNameError(location.to_string()))
            }
        },
        Ok(uuid) => {
            service.find_container_by_uuid(uuid)
        }
    }
}