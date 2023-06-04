//! Functions for interacting with table `accounts`

use super::get_database;

use crate::entities::{account, prelude::*};
use crate::starling::account::Account as StarlingAccount;
use crate::starling::client::{StarlingApiClient, StarlingClient};
use anyhow::Result;
use sea_orm::*;

/// Insert accounts for the Starling account type with the given access token
pub async fn add(token: &String) -> Result<Vec<StarlingAccount>> {
    let db = get_database().await.unwrap();
    let client = StarlingApiClient::new(token);
    let accounts = client.accounts().await;

    println!("Processing {} account(s)", accounts.len());

    for account in accounts.iter() {
        insert_account(account, token, &db).await?;
    }

    Ok(accounts)
}

// /// Insert accounts for the Starling account type with the access tokens in the config file
// pub async fn add_from_config() -> Result<()> {
//     let db = get_database().await.unwrap();
//     let config = Config::new();

//     for item in config.token.unwrap().iter() {
//         for token in item.values() {
//             let client = StarlingApiClient::new(token);
//             for account in client.accounts().await.iter() {
//                 insert_account(account, token, &db).await?;
//             }
//         }
//     }

//     Ok(())
// }

/// If the account doesn't exist in the database, insert it
pub async fn insert_account(
    account: &StarlingAccount,
    token: &String,
    db: &DatabaseConnection,
) -> Result<(), anyhow::Error> {
    match account_exists(&account.uid).await {
        None => {
            let record = make_record(token, account);
            Account::insert(record).exec(db).await?;
        }
        Some(_) => println!("Account exists for token: skipping"),
    }

    Ok(())
}

fn make_record(token: &String, account: &StarlingAccount) -> account::ActiveModel {
    account::ActiveModel {
        token: ActiveValue::Set(token.to_owned()),
        uid: ActiveValue::set(account.uid.to_owned()),
        created_at: ActiveValue::set(account.created_at.to_owned()),
        default_category: ActiveValue::set(account.default_category.to_owned()),
        name: ActiveValue::set(account.name.to_owned()),
        ..Default::default()
    }
}

/// Return true if an account with the given account uid exists in the database.
async fn account_exists(account_uid: &String) -> Option<account::Model> {
    let db = get_database().await.unwrap();
    Account::find()
        .filter(account::Column::Uid.eq(account_uid))
        .one(&db)
        .await
        .expect("getting counterparty id")
}
