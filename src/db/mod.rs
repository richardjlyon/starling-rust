pub mod account;
pub mod counterparty;
pub mod transaction;

use crate::config::Config;
use sea_orm::*;

async fn get_database() -> DatabaseConnection {
    let config = Config::new();
    Database::connect(&config.db_url())
        .await
        .expect("getting database")
}
