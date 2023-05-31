//! Command Line Interface `Database` commands
//!

use std::io::Write;
use std::{io, process};

use crate::entities::account;
use crate::{
    config::Config,
    db,
    starling::client::{StarlingApiClient, StarlingClient},
};
use anyhow::Result;
use sea_orm::*;

/// Initialise the database.
///
/// Drop and reinstate the tables if they exist, fetch account data from Starling and save, and initialise with
/// transaction data for all accounts and saving spaces.
///
pub async fn initialise_database() -> Result<()> {
    let config = Config::new();

    if !proceed() {
        println!("Exiting.");
        process::exit(1);
    }

    // process accounts
    db::account::delete_all().await;
    db::transaction::delete_all().await;
    db::counterparty::delete_all().await;

    // delete the old ones
    let db = Database::connect(&config.db_url())
        .await
        .expect("getting database");

    let res: DeleteResult = account::Entity::delete_many()
        .exec(&db)
        .await
        .expect("deleting accounts");

    for item in config.token.iter() {
        for (name, token) in item.iter() {
            // drop and reload the accounts
            let client = StarlingApiClient::new(token);
            for account in client.accounts().await.iter() {
                db::account::insert_or_update(&account);
            }
        }
    }

    Ok(())
}

// Return true if user enters 'y' or 'Y'
fn proceed() -> bool {
    println!("This will destroy the database and can't be undone");
    print!("Proceed? (y/n) ");
    io::stdout().flush().unwrap();
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("failed to read response");
    response.trim().to_lowercase() == String::from("y")
}
