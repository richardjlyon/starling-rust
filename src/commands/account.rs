//! Command Line Interface `Accounts` commands
//!

use crate::db;
use anyhow::Result;

/// Add the accounts for the given API token
///
/// Store the token in a local token file
pub async fn add(token: &String) -> Result<()> {
    db::account::add(token).await?;
    Ok(())
}

pub async fn add_from_config() -> Result<()> {
    // db::account::add_from_config().await?;
    Ok(())
}

/// Fetch account information from Starling and populate the database
pub async fn get_accounts() -> Result<()> {
    Ok(())
}
