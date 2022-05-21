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
    List (ListCommand),

    /// Remove an object from the catalogue.
    Delete { name_or_id: String }
}

#[derive(Subcommand)]
pub enum AddCommand {
    Item(AddItemArgs),
    Container(AddContainerArgs)
}

// Both AddItemArgs and AddContainerArgs have a custom parser for specifically because of location.
// The complexity comes from cramming the values of two mutually-exclusive flags into one field.

// In the past I had the idea of allowing the user to specify either a uuid or a name for the 
// location of the new object under one command-line parameter, and having the computer guess 
// whether it was a uuid or name. This is possible, and the code to do it isn't nearly as complex,
// but I think it leads to bad UX. For example, what if the user mistyped a uuid? Because the 
// uuid failed to parse, it would get treated as a name and the error would be "Couldn't find name"
// instead of "invalid uuid". Specifying the type as a flag lets the computer provide correct error messages.

#[derive(Debug)]
pub struct AddItemArgs {
    pub name: String,
    pub location: Location,
    pub quantity: u64,
    pub description: Option<String>,
    pub r_type: Option<String>
}

impl FromArgMatches for AddItemArgs {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let name: String = matches.value_of("name")
            .map(|s| s.to_owned())
            .ok_or(clap::Error::raw(clap::ErrorKind::ArgumentNotFound, "Parse Error: Missing required arugment 'name'."))?;
        
        let location = Location::parse_from_matches(matches)?
            .expect("Neither uuid-location nor name-location provided. This error should never happen.");

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

        self.location = Location::parse_from_matches(matches)?
            .expect("Neither uuid-location nor name-location provided. This error should never happen.");

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
        add_item_augment_args(cmd)
    }

    fn augment_args_for_update(cmd: Command<'_>) -> Command<'_> {
        add_item_augment_args(cmd)
    }
}

pub struct AddContainerArgs {
    pub name: String,
    pub location: Option<Location>,
    pub description: Option<String>,
    pub r_type: Option<String>
}

impl FromArgMatches for AddContainerArgs {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let name = matches.value_of("name")
            .map(|s| s.to_owned())
            .ok_or(clap::Error::raw(clap::ErrorKind::ArgumentNotFound, "Parse Error: Missing required argument 'name'."))?;
        
        let location = Location::parse_from_matches(matches)?;

        Ok(AddContainerArgs {
            name,
            location,
            description: matches.value_of("description").map(|v| v.to_owned()),
            r_type: matches.value_of("type").map(|v| v.to_owned())
        })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        self.name = matches.value_of("name")
            .map(|s| s.to_owned())
            .ok_or(clap::Error::raw(clap::ErrorKind::ArgumentNotFound, "Parse Error: Missing required argument 'name'."))?;

        if let Some(location) = Location::parse_from_matches(matches)? {
            self.location = Some(location);
        }

        if let Some(description) = matches.value_of("description") {
            self.description = Some(description.to_owned());
        }

        if let Some(r_type) = matches.value_of("type") {
            self.r_type = Some(r_type.to_owned());
        }

        Ok(())
    }
}

impl Args for AddContainerArgs {
    fn augment_args(cmd: Command<'_>) -> Command<'_> {
        add_container_augment_args(cmd)
    }

    fn augment_args_for_update(cmd: Command<'_>) -> Command<'_> {
        add_container_augment_args(cmd)
    }
}

#[derive(Subcommand)]
pub enum ListCommand {
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
pub enum Location {
    Uuid(Uuid),
    Name(String)
}

impl Location {
    fn parse_from_matches(matches: &clap::ArgMatches) -> Result<Option<Self>, clap::Error> {
        matches.value_of_t::<Uuid>("uuid-location")
            .map(|uuid|  Some(Self::Uuid(uuid)))
            .or_else(|e|
                if e.kind() == ErrorKind::ArgumentNotFound {
                    Ok(matches.value_of("name-location").map(|n| Self::Name(n.to_owned())))
                } else {
                    Err(e)
                }
            )
    }
}

/// Adds the location args and ArgGroup to the `cmd`.
fn add_location_args(cmd: clap::Command<'_>, required: bool) -> clap::Command<'_> {
    cmd
        .args(&[
            arg!(--"uuid-location" <LOCATION> "The uuid of the container in which this item is stored.'")
                .required(false),
            arg!(--"name-location" <LOCATION> "The name of the container in which this item is stored.")
                .required(false)
        ])
        .group(ArgGroup::new("location")
            .args(&["uuid-location", "name-location"])
            .required(required))
}

fn add_item_augment_args(cmd: Command<'_>) -> Command<'_> {
    add_location_args(cmd, true)
            .args(&[
                arg!(-n --name <NAME> "The name of the new item."),
                arg!(-q --quantity [QUANTITY] "The quantity of items. Default is one.")
                    .visible_alias("count")
                    .visible_short_alias('c')
                    .default_value("1"),
                arg!(--description [description] "An optional description of the item."),
                arg!(--"type").takes_value(true)
            ])
}

fn add_container_augment_args(cmd: Command<'_>) -> Command<'_> {
    add_location_args(cmd, false)
            .args(&[
                arg!(-n --name <NAME> "The name of the new container."),
                arg!(--description [DESCRIPTION] "An optional description of the item."),
                arg!(--"type").takes_value(true)
            ])
}