mod jay_data;
mod error;
mod cli;

use std::collections::HashMap;

use clap::StructOpt;
use error::Error;
use jay_data::{JsonDataService, models::{ContainerModel, ItemModel}};
use cli::{Cli, ActionCommand, AddCommand, AddItemArgs, AddContainerArgs};


fn main() {
    let cli = Cli::parse();
    let mut service = JsonDataService::new("jay.json", true).expect("Error reading jay.json.");
    

    match cli.command {
        ActionCommand::Add(add_cmd) => add_object(&mut service, add_cmd).expect("Error creating new object."),
        _ => println!("Not implemented")
    }

    service.flush().expect("Error writing to jay.json.");
}


fn add_object(service: &mut JsonDataService, add_cmd: AddCommand) -> Result<(), Error> {        
    match add_cmd {
        AddCommand::Item (AddItemArgs {name, location, quantity, description, r_type}) => {
            let uuid = service.find_container_by_location(&location)
                .map(|c| c.uuid)?;
            let extras = build_extras(r_type)?;

            let item_model = ItemModel::new(name, description, uuid, quantity, Some(extras));
            Ok(service.models.items.push(item_model))
        }
        AddCommand::Container (AddContainerArgs {name, location, description, r_type}) => {
            let uuid = location
                .as_ref()
                .map(|l| service.find_container_by_location(l).map(|c| c.uuid))
                .transpose()?;
            let extras = build_extras(r_type)?;

            let container_model = ContainerModel::new(name, description, uuid, Some(extras));
            Ok(service.models.containers.push(container_model))
        }
    }
}

fn build_extras(r_type: Option<String>) -> Result<HashMap<String, serde_json::Value>, Error> {
    let type_value = serde_json::to_value(r_type)?;
    Ok(HashMap::from([
        ("type".to_string(), type_value)
    ]))
}
