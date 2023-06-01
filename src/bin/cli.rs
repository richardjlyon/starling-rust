//! `starling` ia a command line application for processing Starling transactions.
//!
//! Transactionsa are stored in a database. Reports can be produced in [ledger](https://ledger-cli.org/features.html) format.

use clap::{arg, Command};
use money::commands::{database::initialise_database, transactions::get_transactions};
use std::process;

use money::starling::client::StarlingApiClient;

/// Commands
///
/// See: https://github.com/clap-rs/clap/blob/master/examples/git.rs
///

fn cli() -> Command {
    Command::new("money")
        .about("A money managing app")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("db")
                .about("Manage database")
                .arg_required_else_help(true)
                .subcommand(Command::new("init").about("Initialise the database")),
        )
        .subcommand(Command::new("accounts").about("Get accounts"))
        .subcommand(Command::new("balances").about("Get balances"))
        .subcommand(
            Command::new("transactions")
                .about("get transactions")
                .arg(arg!(-d [DAYS] "The days to get").default_value("31")),
        )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("db", sub_matches)) => {
            let init_command = sub_matches.subcommand().unwrap();
            match init_command {
                ("init", _) => {
                    println!("Initialising database");
                    if let Err(e) = initialise_database().await {
                        println!("Application error: {}", e);
                        process::exit(1);
                    }
                }
                (name, _) => {
                    unreachable!("Unsupported command `{name}`")
                }
            }
        }

        Some(("accounts", sub_matches)) => println!("Processing accounts"),

        Some(("balances", sub_matches)) => println!("Processing balances"),

        Some(("transactions", sub_matches)) => {
            println!("Processing transactions");
            let days = *sub_matches.get_one::<i64>("DAYS").expect("required");
            if let Err(e) = get_transactions(days).await {
                println!("Application error: {}", e);
                process::exit(1);
            }
        }
        _ => unreachable!(),
    }
}
