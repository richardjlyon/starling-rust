//! `starling` ia a command line application for processing Starling transactions.
//!
//! Transactionsa are stored in a database. Reports can be produced in [ledger](https://ledger-cli.org/features.html) format.
//! seaql, seaorm, sqlx

// mod db;
// mod entities;
// mod starling;

use clap::{Parser, Subcommand};
use std::env;

use money::db;
use money::starling::client::{StarlingApiClient, StarlingClient};

/// Command line arguments
#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

/// Commands
#[derive(Clone, Debug, Subcommand)]
enum Command {
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
    let args = Args::parse();

    let personal_token =
        env::var("PERSONAL_TOKEN").expect("PERSONAL_TOKEN is not set in .env file");
    let client = StarlingApiClient::new(personal_token);
    // let client = StarlingMockClient::new();

    match args.command {
        Command::Balances => todo!(),

        Command::Transactions { days } => {
            if let Some(account) = client.accounts().await.iter().next() {
                db::service::insert_or_update(&client, account, days).await;
            }
        }
    }

    Ok(())
}
