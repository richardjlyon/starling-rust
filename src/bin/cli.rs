//! `starling` ia a command line application for processing Starling transactions.
//!
//! Transactionsa are stored in a database. Reports can be produced in [ledger](https://ledger-cli.org/features.html) format.

use clap::{Parser, Subcommand};
use money::commands::{database::initialise_database, transactions::get_transactions};
use std::process;

use money::starling::client::StarlingApiClient;

/// Command line arguments
#[derive(Parser)]
#[clap(about, version, author)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// Commands
#[derive(Subcommand)]
enum Commands {
    /// Load accounts to the database
    Init,

    /// Account balances
    Balances,

    /// Account Transactions
    Transactions {
        //// Days to get
        #[clap(short, long, default_value_t = 7)]
        days: i64,
    },
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt::init();

    match cli.command {
        Commands::Balances => todo!(),
        Commands::Init => {
            if let Err(e) = initialise_database().await {
                println!("Application error: {}", e);
                process::exit(1);
            }
        }
        Commands::Transactions { days } => {
            if let Err(e) = get_transactions(days).await {
                println!("Application error: {}", e);
                process::exit(1);
            }
        }
    }

    Ok(())
}
