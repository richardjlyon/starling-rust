//! Command Line Interface `Transaction` commands
//!

use std::error::Error;

use crate::{
    db,
    starling::client::{StarlingApiClient, StarlingClient},
};

/// Fetch transactions for the specified number of days and save to the database
pub async fn get_transactions(client: &StarlingApiClient, days: i64) -> Result<(), Box<dyn Error>> {
    if let Some(account) = client.accounts().await.iter().next() {
        db::service::insert_or_update(client, account, days).await;
    }

    Ok(())
}
