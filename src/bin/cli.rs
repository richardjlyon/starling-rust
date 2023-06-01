//! `starling` ia a command line application for processing Starling transactions.
//!
//! Transactionsa are stored in a database. Reports can be produced in [ledger](https://ledger-cli.org/features.html) format.

use anyhow::Result;
use clap::{arg, Command};
use money::commands;
use std::process;

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
            Command::new("admin")
                .about("Administer the application")
                .arg_required_else_help(true)
                .subcommand(Command::new("init").about("Initialise a fresh instance")),
        )
        .subcommand(
            Command::new("account")
                .about("Manage accounts")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("add")
                        .about("Add an account")
                        .arg(arg!(-c --"config" "Initialise from config file"))
                        .arg(arg!(-t --"token" <APITOKEN> "Initialise from api token")),
                ),
        )
        .subcommand(Command::new("balances").about("Get balances"))
        .subcommand(
            Command::new("transactions")
                .about("get transactions")
                .arg(arg!(-d [DAYS] "The days to get").default_value("31")),
        )
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("admin", sub_matches)) => {
            let admin_command = sub_matches.subcommand().unwrap();
            match admin_command {
                ("init", _) => {
                    if let Err(e) = commands::admin::initialise().await {
                        println!("Application error: {}", e);
                        process::exit(1);
                    }
                }
                (name, _) => {
                    unreachable!("Unsupported command `{name}`")
                }
            }
        }

        Some(("account", sub_matches)) => {
            let account_command = sub_matches.subcommand().unwrap();
            match account_command {
                ("add", sub_matches) => {
                    let from_config = sub_matches.get_one::<bool>("config");
                    let token = sub_matches.get_one("token");
                    match from_config {
                        Some(_) => {
                            println!("Getting from config");
                            commands::account::add_from_config().await?;
                        }
                        None => match token {
                            Some(token) => {
                                println!("Adding from token");
                                commands::account::add(token).await?;
                            }
                            None => {
                                println!("No options selected");
                                process::exit(1);
                            }
                        },
                    }
                }
                (name, _) => {
                    unreachable!("Unsupported command `{name}`")
                }
            }
        }

        Some(("balances", _sub_matches)) => println!("Processing balances"),

        Some(("transactions", sub_matches)) => {
            println!("Processing transactions");
            let days = *sub_matches.get_one::<i64>("DAYS").expect("required");
            if let Err(e) = commands::transactions::update(days).await {
                println!("Application error: {}", e);
                process::exit(1);
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
