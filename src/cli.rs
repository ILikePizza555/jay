use clap::{Parser, Subcommand, Args, FromArgMatches, arg, ArgGroup, Command, ErrorKind};
use uuid::Uuid;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: ActionCommand
}

#[derive(Subcommand)]
pub enum ActionCommand {
    /// Create and insert a new object into the catalogue. 
    #[clap(subcommand)]
    Add (AddCommand),

    /// List objects in the catalogue.
    #[clap(subcommand)]
    List (ListCommands),

    /// Remove an object from the catalogue.
    Delete { name_or_id: String }
}

#[derive(Subcommand)]
pub enum AddCommand {
    Item(AddItemArgs),
    Container(AddContainerArgs)
}

#[derive(Debug)]
pub struct AddItemArgs {
    name: String,
    location: Location,
    quantity: u64,
    description: Option<String>,
    r_type: Option<String>
}

impl FromArgMatches for AddItemArgs {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let name: String = matches.value_of("name")
            .map(|s| s.to_owned())
            .ok_or(clap::Error::raw(clap::ErrorKind::ArgumentNotFound, "Parse Error: Missing required arugment 'name'."))?;
        
        let location = Location::parse_from_matches(matches)?;

        let quantity: u64 = matches.value_of_t("quantity")?;

        Ok(Self {
            name,
            location,
            quantity,
            description: matches.value_of("description").map(|v| v.to_owned()),
            r_type: matches.value_of("type").map(|v| v.to_owned())
        })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        self.name = matches.value_of("name")
            .map(|s| s.to_owned())
            .ok_or(clap::Error::raw(clap::ErrorKind::ArgumentNotFound, "Parse Error: Missing required argument 'name'."))?;

        self.location = Location::parse_from_matches(matches)?;
        self.quantity = matches.value_of_t("quantity")?;
        
        if let Some(description) = matches.value_of("description") {
            self.description = Some(description.to_owned());
        }

        if let Some(r_type) = matches.value_of("type") {
            self.r_type = Some(r_type.to_owned());
        }

        Ok(())
    }
}

impl Args for AddItemArgs {
    fn augment_args(cmd: clap::Command<'_>) -> clap::Command<'_> {
        cmd
            .args(&[
                arg!(-n --name <NAME> "The name of the new item."),
                arg!(--"uuid-location" <LOCATION> "The uuid of the container in which this item is stored.").required(false),
                arg!(--"name-location" <LOCATION> "The name of the container in which this item is stored.").required(false),
                arg!(-q --quantity [QUANTITY] "The quantity of items. Default is one.")
                    .visible_alias("count")
                    .visible_short_alias('c')
                    .default_value("1"),
                arg!(--description [description] "An optional description of the item."),
                arg!(--"type").takes_value(true)
            ])
            .group(ArgGroup::new("location")
                .args(&["uuid-location", "name-location"])
                .required(true))
    }

    fn augment_args_for_update(cmd: Command<'_>) -> Command<'_> {
        cmd
            .args(&[
                arg!(-n --name <name> "The name of the new item."),
                arg!(--uuid-location <location> "The uuid of the container in which this item is stored."),
                arg!(--name-location <name> "The name of the container in which this item is stored."),
                arg!(-q --quantity [quantity] "The quantity of items.")
                    .visible_alias("count")
                    .visible_short_alias('c')
                    .default_value("1"),
                arg!(--description [description] "An optional description of the item."),
                arg!(--"type").takes_value(true)
            ])
            .group(ArgGroup::new("location")
                .args(&["uuid-location", "name-location"])
                .required(true))
    }
}

pub struct AddContainerArgs {
    name: String,
    location: Location,
    description: Option<String>,
    r_type: Option<String>
}

impl FromArgMatches for AddContainerArgs {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        todo!()
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        todo!()
    }
}

impl Args for AddContainerArgs {
    fn augment_args(cmd: Command<'_>) -> Command<'_> {
        cmd
            .args(&[
                arg!(-n --name <name> "The name of the new container."),
                arg!(--description [description] "An optional description of the item."),
                arg!(--"type").takes_value(true)
            ])
    }

    fn augment_args_for_update(cmd: Command<'_>) -> Command<'_> {
        todo!()
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

/// Differentiates between a uuid or a name for setting a location
#[derive(Debug)]
enum Location {
    Uuid(Uuid),
    Name(String)
}

impl Location {
    fn parse_from_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        matches.value_of_t::<Uuid>("uuid-location")
            .map(|uuid|  Self::Uuid(uuid))
            .or_else(|e|
                if e.kind() == ErrorKind::ArgumentNotFound {
                    let name = matches.value_of("name-location")
                        .expect("Cannot parse AddItemArgs: Neither a uuid-location nor a name-location was provided.");
                    Ok(Self::Name(name.to_owned()))
                } else {
                    Err(e)
                }
            )
    }
}