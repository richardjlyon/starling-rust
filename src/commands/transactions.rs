//! Command Line Interface `Transaction` commands
//!

use crate::db;
use anyhow::Result;

/// Fetch transactions for the specified number of days and save to the database
pub async fn get_transactions(days: i64) -> Result<()> {
    db::transaction::insert_or_update(days).await;

    Ok(())
}
