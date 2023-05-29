//! `starling` ia a command line application for processing Starling transactions.
//!
//! Transactionsa are stored in a database. Reports can be produced in [ledger](https://ledger-cli.org/features.html) format.

use clap::{Parser, Subcommand};
use std::env;

use money::db;
use money::starling::client::{StarlingApiClient, StarlingClient};

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
    Accounts,

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
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let personal_token =
        env::var("PERSONAL_TOKEN").expect("PERSONAL_TOKEN is not set in .env file");
    let client = StarlingApiClient::new(personal_token);
    // let client = StarlingMockClient::new();

    match cli.command {
        Commands::Accounts => todo!(),

        Commands::Balances => todo!(),

        Commands::Transactions { days } => {
            if let Some(account) = client.accounts().await.iter().next() {
                db::service::insert_or_update(&client, account, days).await;
            }
        }
    }

    Ok(())
}
