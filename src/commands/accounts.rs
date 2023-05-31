//! Command Line Interface `Accounts` commands
//!

use anyhow::Result;

use crate::{
    db,
    starling::client::{StarlingApiClient, StarlingClient},
};

/// Fetch account information from Starling and populate the database
pub async fn get_accounts() -> Result<()> {
    Ok(())
}
