pub mod account;
pub mod counterparty;
pub mod transaction;
use anyhow::Result;
use sea_orm_migration::prelude::*;

use crate::config::Config;
use migration::Migrator;
use sea_orm::*;

async fn get_database() -> DatabaseConnection {
    let config = Config::new();
    Database::connect(&config.db_url())
        .await
        .expect("getting database")
}

/// Reset the database
///
/// This will delete all tables and recreate them
pub async fn reset() -> Result<()> {
    let db = get_database().await;
    Migrator::fresh(&db).await?;

    Ok(())
}
