mod jay_data;
mod error;
mod cli;

use std::collections::HashMap;

use clap::StructOpt;
use error::Error;
use jay_data::{JsonDataService, models::{ContainerModel, ItemModel}};
use cli::{Cli, ActionCommand, AddCommand};
use uuid::Uuid;


fn main() {
    let cli = Cli::parse();
    let mut service = JsonDataService::new("jay.json", true).expect("Error reading jay.json.");
    

    match cli.command {
        //ActionCommand::Add(add_cmd) => add_object(&mut service, add_cmd).expect("Error creating new object."),
        ActionCommand::Add(AddCommand::Item(args)) => println!("{:?}", args),
        _ => println!("Not implemented")
    }

    service.flush().expect("Error writing to jay.json.");
}

/*
fn add_object(service: &mut JsonDataService, add_cmd: AddCommand) -> Result<(), Error> {
    let location_uuid = get_uuid_from_location(service, add_cmd.get_location())?;
    
    let type_value = serde_json::to_value(add_cmd.get_type())?;
    let extras = HashMap::from([
        ("type".to_string(), type_value)
    ]);

    match add_cmd {
        AddCommand::Item { name, location: _, quantity, description, r_type: _ } => {
            let uuid = location_uuid.ok_or(Error::UuidRequiredError())?;
            let item_model = ItemModel::new(name, description, uuid, quantity, Some(extras));

            Ok(service.models.items.push(item_model))
        }
        AddCommand::Container {name, location: _, description, r_type: _} => {
            let container_model = ContainerModel::new(name, description, location_uuid, Some(extras));
            Ok(service.models.containers.push(container_model))
        }
    }
}

fn get_uuid_from_location(service: &JsonDataService, location: Option<&str>) -> Result<Option<Uuid>, Error> {
    location
        .map(|l| get_container_by_location(service, l)
            .map(|c| c.uuid)
        )
        .transpose()
}

/// Checks if the provided location is a uuid. 
/// If it is, then searches containers for a match. 
/// Otherwise searches containers for one with the same name as location.
/// If multiple are found, an error is returned. 
fn get_container_by_location<'l>(service: &'l JsonDataService, location: Location) -> Result<&'l ContainerModel, Error> {
   match location {
       Location::Name(name)
   }
   
    match <uuid::Uuid as std::str::FromStr>::from_str(location) {
        Err(_) => {
            let name_matches: Vec<&ContainerModel> = service.filter_containers_by_name(location);
            
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
} */