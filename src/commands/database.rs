//! Command Line Interface `Database` commands
//!

use crate::db;
use anyhow::Result;
use std::io::Write;
use std::{io, process};

/// Initialise the database.
///
/// Drop and reinstate the tables if they exist, fetch account data from Starling and save, and initialise with
/// transaction data for all accounts and saving spaces.
///
pub async fn initialise() -> Result<()> {
    if !proceed() {
        println!("Exiting.");
        process::exit(1);
    }

    db::reset().await?;

    // db::account::initialise().await?;

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
