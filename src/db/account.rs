//! Functions for interacting with table `accounts`

use super::get_database;
use crate::entities::{account, prelude::*};
use crate::starling::client::StarlingApiClient;
use crate::{config::Config, starling::client::StarlingClient};
use anyhow::Result;
use sea_orm::*;

// DELETE * FROM account;
pub async fn delete_all() -> Result<()> {
    let db = get_database().await;
    account::Entity::delete_many().exec(&db).await?;

    Ok(())
}

// populate accounts table
pub async fn initialise() -> Result<()> {
    let db = get_database().await;
    let config = Config::new();
    for item in config.token.iter() {
        for token in item.values() {
            let client = StarlingApiClient::new(token);
            for account in client.accounts().await.iter() {
                let record = account::ActiveModel {
                    account_uid: ActiveValue::set(account.account_uid.to_owned()),
                    created_at: ActiveValue::set(account.created_at.to_owned()),
                    default_category: ActiveValue::set(account.default_category.to_owned()),
                    name: ActiveValue::set(account.name.to_owned()),
                    ..Default::default()
                };
                Account::insert(record).exec(&db).await?;
            }
        }
    }

    Ok(())
}
