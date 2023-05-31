//! Functions for interacting with table `accounts`

use super::get_database;
use crate::entities::account;
use crate::starling::account::Account;
use sea_orm::*;

// DELETE * FROM account;
pub async fn delete_all() {
    let db = get_database().await;
    account::Entity::delete_many()
        .exec(&db)
        .await
        .expect("deleting accounts");
}

pub async fn insert_or_update(account: &Account) {
    let db = get_database().await;

    println!("{:#?}", account);
}
