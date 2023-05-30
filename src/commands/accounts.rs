//! Command Line Interface `Accounts` commands
//!

use std::error::Error;

use crate::{
    db,
    starling::client::{StarlingApiClient, StarlingClient},
};

/// Fetch account information from Starling and populate the database
pub async fn get_accounts(client: &StarlingApiClient) -> Result<(), Box<dyn Error>> {
    Ok(())
}
