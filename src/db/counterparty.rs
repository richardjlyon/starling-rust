use super::get_database;
use crate::entities::counterparty;
use sea_orm::*;

// DELETE * FROM account;
pub async fn delete_all() {
    let db = get_database().await;
    counterparty::Entity::delete_many()
        .exec(&db)
        .await
        .expect("deleting accounts");
}
