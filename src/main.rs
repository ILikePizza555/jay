use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use rusqlite::{Connection, Statement};
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

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let db_connection = Connection::open("./jay.db")?;

    match args.command {
        Commands::List => {
            // query_map requires a mutable reference so we let mut here
            let mut statement = db_connection.prepare(
                r#"SELECT item_history_id, last_modified, uuid, name, quantity, status, deleted
                        FROM current_items
                        WHERE deleted = 0"#)?;

            // query_map creates an interator of Results of a tuple with our data.
            // filter_map filters out the errors from the iterator while printing them to the stderr
            let mapped_rows = statement.query_map([], |row| Ok((
                row.get::<usize, u64>(0)?,
                row.get::<usize, DateTime<Utc>>(1)?,
                row.get::<usize, Uuid>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, u64>(4)?,
                row.get::<usize, String>(5)?,
            )))?.filter_map(|result| result.map_err(|e| {eprintln!("{}", e)}).ok());

            // Finally, output the data to stdout.
            // This is a very simplistic way to format a table but it works for now.
            for (item_history_id, last_modified, uuid, name, quantity, status) in mapped_rows {
                // TODO: Can I do this by directly passing the tuple to println! somehow?
                println!(
                    "{0: <3} {1:?} {2:?} {3: <3} {4: <10} {5}",
                    item_history_id, last_modified, uuid, quantity, status, name
                );
            }

            Ok(())
        },
        Commands::History { uuid } => {
            // TODO: Make a macro or something that can generate a selection query and output the result to a tuple.
            // Basically meld these lines into one
            let mut statement = db_connection.prepare(
            r#"SELECT id, items_history.'from', items_history.'to', who, name, description, type, quantity, status, deleted 
                    FROM items_history WHERE uuid = (?1)"#)?;
            
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

            //TODO: Show the difference with colors
            for (id, from, to, who, name, description, r_type, quantity, status, deleted) in mapped_rows {
                println!(
                    "{0: <3} {1:?} {2:?} {3: <10}\t{4:} {5:} {6:} {7:} {8:} {9:}",
                    id, from, to, who, name, description, r_type, quantity, status, deleted)
            }

            Ok(())
        }
        _ => {
            println!("Not implemented.");
            Ok(())
        }
    }
}