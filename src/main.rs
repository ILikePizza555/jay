mod data;

use std::collections::HashMap;

use clap::{Parser, Subcommand};
use data::JsonDataService;

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
    let mut data = JsonDataService::new("jay.json", true).expect("Error reading jay.json.");

    match cli.command {
        ActionCommands::Add(AddCommands::Container { name, location, description, r_type }) => {
            let location_value = serde_json::to_value(location).expect("Error parsing location parameter.");
            let r_type_value = serde_json::to_value(r_type).expect("Error parsing type parameter.");
            let extras = HashMap::from([
                ("location".to_string(), location_value),
                ("type".to_string(), r_type_value)
            ]);

            data.data.containers.push(data::ContainerModel::new(name, description, Some(extras)));
        },
        _ => println!("Not implemented")
    }
}
