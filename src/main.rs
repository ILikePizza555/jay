mod jay_data;
mod error;

use std::collections::HashMap;

use clap::{Parser, Subcommand};
use error::Error;
use jay_data::{JsonDataService, models::ContainerModel};

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
        quantity: usize,

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
        ActionCommands::Add(AddCommands::Container { name, location, description, r_type }) => {
            let location_uuid = evert(location.map(|uuid_str| {
                service.find_container_by_uuid_str(&uuid_str).map(|v| v.uuid)
            })).expect("Error in provided location.");

            let r_type_value = serde_json::to_value(r_type).expect("Error parsing type parameter.");
            let extras = HashMap::from([
                ("type".to_string(), r_type_value)
            ]);
            let container_model = ContainerModel::new(name, description, location_uuid, Some(extras));

            service.models.containers.push(container_model)
        },
        _ => println!("Not implemented")
    }

    service.flush().expect("Error writing to jay.json.");
}

fn evert<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}