mod db;

use clap::{Parser, Subcommand};
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

fn main() {
    
}
