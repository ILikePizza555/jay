use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use rusqlite::Connection;
use uuid::Uuid;

#[derive(Debug, Parser)]
struct CliArgs {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add {
        #[clap(short, long)]
        name: String,

        #[clap(short, long)]
        description: Option<String>,

        #[clap(name = "type", short, long)]
        r_type: Option<String>,

        #[clap(short, long, default_value_t = 1)]
        quantity: u64,

        #[clap(short, long)]
        status: Option<String>,
    },
    Update {
        #[clap(short, long)]
        name: Option<String>,

        #[clap(short, long)]
        description: Option<String>,

        #[clap(name = "type", short, long)]
        r_type: Option<String>,

        #[clap(short, long)]
        quantity: Option<u64>,

        #[clap(short, long)]
        status: Option<String>,
    },
    List,
    History {
        uuid: Uuid
    },
}

#[derive(Debug)]
struct CurrentItemViewDTO {
    item_history_id: u64,
    last_modified: DateTime<Utc>,
    uuid: Uuid,
    name: String,
    description: String,
    r_type: String,
    quantity: u64,
    status: String,
    deleted: bool
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let db_connection = Connection::open("./jay.db")?;

    match args.command {
        Commands::List => {
            // query_map requires a mutable reference so we let mut here
            let mut statement = db_connection.prepare(
            r#"SELECT item_history_id, last_modified, uuid, name, description, type, quantity, status, deleted
                    FROM current_items
                    WHERE deleted = 0"#)?;

            // query_map creates an iterator of Result<CurrentItemViewDTO>
            // filter_map filters out the errors from the iterator while printing them to the stderr
            let mapped_rows = statement.query_map([], |row| Ok(CurrentItemViewDTO {
                item_history_id:    row.get(0)?,
                last_modified:      row.get(1)?,
                uuid:               row.get(2)?,
                name:               row.get(3)?,
                description:        row.get(4)?,
                r_type:             row.get(5)?,
                quantity:           row.get(6)?,
                status:             row.get(7)?,
                deleted:            row.get(8)?
            }))?.filter_map(|result| result.map_err(|e| {eprintln!("{}", e)}).ok());

            // Finally, output the data to stdout.
            // This is a very simplistic way to format a table but it works for now.
            for CurrentItemViewDTO { item_history_id, last_modified, uuid, quantity, status, name, .. } in mapped_rows {
                println!(
                    "{0: <3} {1:?} {2:?} {3: <3} {4: <10} {5}",
                    item_history_id, last_modified, uuid, quantity, status, name
                );
            }

            Ok(())
        },
        Commands::History { uuid } => {
            let mut statement = db_connection.prepare(
            r#"SELECT id, items_history.'from', to, who, name, description, type, quantity, status, deleted"
                    FROM items_hisory
                    WHERE uuid = (?1)"#)?;
            
            let mapped_rows = statement.query_map([uuid], |row| Ok((
                row.get::<usize, u64>(0)?,
                row.get::<usize, DateTime<Utc>>(1)?,
                row.get::<usize, Option<DateTime<Utc>>>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, String>(4)?,
                row.get::<usize, String>(5)?,
                row.get::<usize, String>(6)?,
                row.get::<usize, u64>(7)?,
                row.get::<usize, String>(8)?,
                row.get::<usize, bool>(9)?
            )))?.filter_map(|result| result.map_err(|e| {eprintln!("{}", e)}).ok());

            Ok(())
        },
        _ => {
            println!("Not implemented.");
            Ok(())
        }
    }
}