use std::error::Error;

use clap::{Parser, Subcommand};
use rusqlite::Connection;

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

    let db_connection = Connection::open("./jay.db").expect("Could not connect to database!");

    match cli.command {
        ActionCommands::List(ListCommands::All) => {
            let mut statement = db_connection.prepare(
            r#"SELECT 'item' as object_type, uuid, name, description, type, created_date FROM items
                    UNION
                    SELECT 'container' as object_type, uuid, name, description, type, created_date FROM containers;"#)
                .expect("Error creating prepared statement.");
                
            let mut rows = statement.query([]).expect("Error executing query.");
            while let Some(row) = rows.next().unwrap() {
                println!("{} {} {}", row.get_unwrap::<usize, String>(0), row.get_unwrap::<usize, String>(1), row.get_unwrap::<usize, String>(2))
            }
        },
        _ => (),
    }
}
