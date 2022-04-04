use clap::{Parser, Subcommand};
use db::DatabaseConnection;

mod db;

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
    All,
    Container { name_or_id: String }
}

fn main() {
    let cli = Cli::parse();

    let db_connection = DatabaseConnection::open("./jay.db").expect("Could not connect to database!");

    match cli.command {
        ActionCommands::List(ListCommands::All) => {
            let items = db_connection.select_all_items_and_containers().expect("Error executing query.");

            for item in items {
                match item {
                    db::ItemOrContainerRow::Item(i) => println!("Item: {:?}", i),
                    db::ItemOrContainerRow::Container(c) => println!("Container: {:?}", c),
                }
            }
        },
        _ => (),
    }
}
