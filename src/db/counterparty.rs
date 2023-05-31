use super::get_database;
use crate::entities::counterparty;
use anyhow::Result;
use sea_orm::*;

// DELETE * FROM account;
pub async fn delete_all() -> Result<()> {
    let db = get_database().await;
    counterparty::Entity::delete_many().exec(&db).await?;

    Ok(())
}
