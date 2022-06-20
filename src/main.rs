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

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let db_connection = Connection::open("./jay.db")?;

    match args.command {
        Commands::List => {
            let mut statement = db_connection.prepare(
                r#"SELECT item_history_id, last_modified, uuid, name, quantity, status, deleted
                        FROM current_items
                        WHERE deleted = 0"#)?;
            let mapped_rows = statement.query_map([], |row| Ok((
                row.get::<usize, u64>(0)?,
                row.get::<usize, DateTime<Utc>>(1)?,
                row.get::<usize, Uuid>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, u64>(4)?,
                row.get::<usize, String>(5)?,
            )))?.filter_map(|result| result.map_err(|e| {eprintln!("{}", e)}).ok());

            for (item_history_id, last_modified, uuid, name, quantity, status) in mapped_rows {
                println!(
                    "{0: <3} {1:?} {2:?} {3: <3} {4: <10} {5}",
                    item_history_id, last_modified, uuid, quantity, status, name
                );
            }

            Ok(())
        },
        _ => {
            println!("Not implemented.");
            Ok(())
        }
    }
}