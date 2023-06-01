//! Command Line Interface `Admin` commands
//!

use crate::config::Config;
use crate::db::{self, get_database};
use anyhow::Result;
use colored::Colorize;
use sea_orm::Database;
use std::io::Write;
use std::{fs, io, process};

/// Initialise the application.
///
/// Create the database and tables
/// Create an empty config file
/// Get token info from the user, fetch account data from Starling and save
/// Get transaction data for all accounts and saving spaces.
///
pub async fn initialise() -> Result<()> {
    if !proceed() {
        println!("Exited.");
        process::exit(1);
    }

    println!("Initialising the application");

    // delete the old config file, if it exists
    // Config::delete();

    // get and verify the database credentials from user
    // match get_db_credentials().await {
    //     Ok(_) => println!("{}", "OK: credentials".green()),
    //     Err(e) => {
    //         println!("{}: {}", "ERROR: Invalid database credentials".red(), e);
    //         println!("Exited");
    //         process::exit(1);
    //     }
    // }

    // reset the database
    // match db::reset().await {
    //     Ok(_) => println!("{}", "OK: reset".green()),
    //     Err(e) => {
    //         println!("{}: {}", "ERROR: Failed to reset the database".red(), e);
    //         println!("Exited");
    //         process::exit(1);
    //     }
    // }

    // get tokens and initialise account info
    match get_accounts().await {
        Ok(_) => println!("{}", "OK: accounts added".green()),
        Err(e) => {
            println!("{}: {}", "ERROR: Failed to get accounts".red(), e);
            println!("Exited");
            process::exit(1);
        }
    }

    // Get transactions

    // Save config to yaml

    Ok(())
}

// Return true if user enters 'y' or 'Y'
fn proceed() -> bool {
    println!(
        "{}",
        "WARNING: This will destroy the database and can't be undone".red()
    );
    print!("Proceed? (y/N) ");
    io::stdout().flush().unwrap();
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("failed to read response");
    response.trim().to_lowercase() == String::from("y")
}

// Get the database credentials from the user
async fn get_db_credentials() -> Result<()> {
    let mut config = Config::new();

    config.db.name = get_string("database name");
    config.db.user = get_string("database user");
    config.db.password = get_string("database password");

    Database::connect(&config.db_url()).await?;

    config.save();

    Ok(())
}

// Get the Starling account token(s) from the user
async fn get_accounts() -> Result<()> {
    let filepath = get_string("Path to Starling API token");
    let contents = fs::read_to_string(filepath).expect("opening file");
    let token = contents.trim().to_string();
    super::account::add(&token).await?;

    Ok(())
}

// Get the Starling account tokens from the user
// fn get_starling_tokens() -> Config {}

fn get_string(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}: ", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
